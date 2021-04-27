use super::{PokSignatureProof, PublicKey, Signature};
use crate::core::*;
use bls12_381_plus::{G1Projective, G2Affine, G2Projective, Scalar};
use digest::Update;
use group::Curve;

/// Proof of Knowledge of a Signature that is used by the prover
/// to construct `PoKOfSignatureProof`.
pub struct PokSignature {
    secrets: Vec<Scalar>,
    proof: ProofCommittedBuilder<G2Projective, G2Affine>,
    commitment: G2Projective,
    sigma_1: G1Projective,
    sigma_2: G1Projective,
}

impl PokSignature {
    /// Creates the initial proof data before a Fiat-Shamir calculation
    pub fn init(
        signature: Signature,
        public_key: &PublicKey,
        messages: &[ProofMessage],
    ) -> Result<Self, Error> {
        if public_key.y.len() < messages.len() {
            return Err(Error::new(1, "mismatched messages and generators"));
        }
        let mut rng = rand::thread_rng();

        let r = Nonce::random(&mut rng);
        let t = Nonce::random(&mut rng);

        // ZKP for signature
        let sigma_1 = signature.sigma_1 * r.0;
        let sigma_2 = (signature.sigma_2 + (signature.sigma_1 * t.0)) * r.0;

        // Prove knowledge of m_tick, m_1, m_2, ... for all hidden m_i and t in J = Y_tilde_1^m_1 * Y_tilde_2^m_2 * ..... * g_tilde^t
        let mut proof = ProofCommittedBuilder::new(G2Projective::sum_of_products_in_place);
        let mut points = Vec::with_capacity(messages.len() + 2);
        let mut secrets = Vec::with_capacity(messages.len() + 2);

        proof.commit_random(G2Projective::generator(), &mut rng);
        points.push(G2Projective::generator());
        secrets.push(t.0);

        proof.commit_random(public_key.w, &mut rng);
        points.push(public_key.w);
        secrets.push(signature.m_tick);

        for (i, _) in messages.iter().enumerate() {
            match messages[i] {
                ProofMessage::Hidden(HiddenMessage::ProofSpecificBlinding(m)) => {
                    proof.commit_random(public_key.y[i], &mut rng);
                    points.push(public_key.y[i]);
                    secrets.push(m.0);
                }
                ProofMessage::Hidden(HiddenMessage::ExternalBlinding(m, n)) => {
                    proof.commit(public_key.y[i], n.0);
                    points.push(public_key.y[i]);
                    secrets.push(m.0);
                }
                _ => {}
            }
        }
        let commitment = G2Projective::sum_of_products_in_place(points.as_ref(), secrets.as_mut());
        Ok(Self {
            secrets,
            commitment,
            proof,
            sigma_1,
            sigma_2,
        })
    }

    /// Convert the committed values to bytes for the fiat-shamir challenge
    pub fn add_proof_contribution(&mut self, hasher: &mut impl Update) {
        hasher.update(self.sigma_1.to_affine().to_uncompressed());
        hasher.update(self.sigma_2.to_affine().to_uncompressed());
        hasher.update(self.commitment.to_affine().to_uncompressed());
        self.proof.add_challenge_contribution(hasher);
    }

    /// Generate the Schnorr challenges for the selective disclosure proofs
    pub fn generate_proof(self, challenge: Challenge) -> Result<PokSignatureProof, Error> {
        let proof = self
            .proof
            .generate_proof(challenge.0, self.secrets.as_ref())?
            .iter()
            .map(|s| Challenge(*s))
            .collect();
        Ok(PokSignatureProof {
            sigma_1: self.sigma_1,
            sigma_2: self.sigma_2,
            commitment: self.commitment,
            proof,
        })
    }
}
