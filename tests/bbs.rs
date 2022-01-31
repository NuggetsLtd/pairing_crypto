use core::convert::TryFrom;
use digest::{ExtendableOutput, Update, XofReader};
use pairing_crypto::bls12_381::{bbs, Challenge, HiddenMessage, Message, Nonce, ProofMessage};
use rand::SeedableRng;

const TEST_KEYS: [&[u8]; 7] = [
    b"",
    b"abc",
    b"abcdefgh",
    b"abcdefghijklmnopqrstuvwxyz",
    b"qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq",
    b"12345678901234567890123456789012345678901234567890",
    b"1qaz2wsx3edc4rfv5tgb6yhn7ujm8ik,9ol.0p;/",
];
const TEST_CLAIMS: [&[u8]; 6] = [
    b"first_name",
    b"surname",
    b"date_of_birth",
    b"father",
    b"mother",
    b"credential_id",
];

const EXPECTED_SIGS: [&str; 7] = [
"b6fbba2938433819c56fcdd54622085a0c5c27234ce1653832dc372803c4cc9b076cc9d2addf0c53e8b532639465428e3e2b8316c42a79ce9b567655403b8fcb2d091c85441f95c9473317f05e8d1bc56309b322a467ceedda700da7d09cacd380a0058e0d5e580b6be8d02550865a2b",
"a5582c36299070118196b12cfc11adfb99dba1609ab6a4008adf77e460308a014bf2ded471a7cca533d3b3ec5eca933c2fe0d510be050a7ae6ecf22c7ea115c48f554a72b784a52200878edd75352dc1056e1911d001e0fee08a6089e0fc6dd15ba81a525345c2e0054289ed34b020e7",
"ae2df35f3ff0256735fa4a43b9f303a920e9a09e5e02028297e97ae9d487fa73266d6dc3f975742a3a19f4ac31a1ae1a666b00b4774105c276f23a70ef8e0087c0746a4c8953cb9a9e32f2ba45c3eeb32bf682419f2dc9e1c8fabaf9e833266aad4dce473d70631daeecf4bf641acb24",
"b9e4676b356a24f353bd30acdd0789b9f56861f34e1486cce7ec506b2fa853a2009dae6b99b4cf1973dfa6bdb16d07c3635867cc1a80fc1653cbd7dbbb5b74694c212bf2368921ba4a967af65bd516f46f691d278b0911d007f5ef46856b3789661276468896f635153952ca63a6ca09",
"8164165f59425877b49944e0dcd86dd966b86d68f3c4cb8da9469d57bff0d410c0eb5e16036e70132446f7b7701995594a8153a5d313f6628c9b132a8b4470609aca62762307a5d31a82117503bd78280e392d7397ab0481131d12d4bcbb49aba9c943500e4c6d96a39c33de19c97e16",
"97934727401b6847f9ec5e703dad99a74fee2adc56ef0bf3480ea221db4d6e997325276668a4518103648f569fc3594535936e98f0dc801c6206a228dde9d75134a7c7ef44f7456e71a1ad82f152ff9006b7d02001a8f874f8850e22fc2d88df26e0cff07859c856a7b63137cc3dcd26",
"8c2edadebc6f8324f416904b72914e2cfe403836efd5d9c7b1c58488f58063e324d6e08560937f56c41739abff1f779d22fcf8b677b3bd365a5cb69fa91478cfdf9602f11b2674992f79e0143ad84bf34b3c993c9979c43abfc45fd59415fd538839890258b676e0690bfc18d1cb1340",
];

#[test]
fn signing() {
    let test_atts = TEST_CLAIMS
        .iter()
        .map(|b| Message::hash(b))
        .collect::<Vec<Message>>();

    for i in 0..TEST_KEYS.len() {
        let sk_opt = bbs::SecretKey::from_seed(TEST_KEYS[i]);
        assert!(sk_opt.is_some());
        let sk = sk_opt.unwrap();
        let pk = bbs::PublicKey::from(&sk);
        let gens = bbs::MessageGenerators::from_public_key(pk, test_atts.len());
        let sig_res = bbs::Signature::new(&sk, &gens, &test_atts);
        assert!(sig_res.is_ok());

        let sig = sig_res.unwrap();
        let cte_sig = bbs::Signature::from_bytes(
            &<[u8; bbs::Signature::BYTES]>::try_from(hex::decode(EXPECTED_SIGS[i]).unwrap())
                .unwrap(),
        );
        let e_sig = cte_sig.unwrap();
        assert_eq!(sig, e_sig);
    }
}

#[test]
fn proofs() {
    use pairing_crypto::MockRng;
    let mut rng = MockRng::from_seed([1u8; 16]);
    let test_atts = TEST_CLAIMS
        .iter()
        .map(|b| Message::hash(b))
        .collect::<Vec<Message>>();

    let expected_proofs = [
    [
        "823a7608249606f116e167960de09cc998ec51f2eaaf4ed72b8e9f4c2eb9bef6df41a29764b726fc1b15510f4ad922ae8bfedfccd815fc7501a1bc973d0e70295b3ace3bbdeb03ad4a3e3e242694379c5c678ffca1f05d098e6dadf675ec1adc85d5577ce30ef56f0db4e71bdeaf5577b952becdba4293d0fdad9750102fcf92c68b64c5f92a4f8ab52a5aa6b4043f986b33cf62fa713b418a9028aee65ea60fb3c673381f3d335a6279a75b31fcf7593bafc83bd971f2305503c930f7fefc4f2f8b8fb62c3a47247495d0949c421c5027cc80e319d4b148c93ae6fb0c77bc69eae2dd346dde10b492d67e226bc5fc1d434ec68f5fa0e4928cecfb380b26995c98861805c175a3607618580edda3a028698bcecdae288e85eacaac95d6806d5e916e94e0bec4bf91e0589ff8eeca589127bff95e7f209a13e2f51263687bd47a68880c27ffbd3ec1008e1b1eec5cf5c704df0ff265c6a7355229b89991e79a65ea395d140a52321e6bce4b3612b3db2b5bf2fab4ca1a7c367aaf87e2bc507dc1b691f7136e52248232a2e397b35a424b0f5ee8dd028047cba4fe3df2d0f1b0d6f8931a062a76b060f6264d778a8572915c431347b341a521062dd4717c3913bd84501efa0eac79218e758f5db3015db9",
        "83a4d06385e6e729307863688e2123d61cb29feb25c8399a6818fd4f8fa417790d0ce24d77258faabc7492507a9dd97882a9d0cb64e8b881d46e5b7b363e4c2975cc95dacbe7e78e7594b5524b6920de04a06b7c198331727c28d8348ceaebb386b0acaa0e9212123a26694cbc0247831cad3188dec719ffb133129f1cb71282f57d8c1788e8bf417a99e31764273f6b41a4a5a28891e8c360901bfefad72f7a1be760be472281239b0629f80259ab5a505bbe26ea04344255d58739a89cc99b1626ced75e5d4d4a04a1f7d2f6ec7a770f997f737357208a40fc3ac2f3ec103c756e5b7d7efa085c4f79da5315c7b72908a4cac386c8dc189db43a2174dc2d1fc1856fe0dbd32754741fa478fe96a1df1e2532102f41099149d939533136caac06f44b767a01836132fb20fd326641ad6e718dd2bf13cf03b1f5feaecb24a9ca853f6b03670cdee6c16ae763154f80b44dbdd1560d7bfe95d584e38c1b6c3a4088d25a87c5f7af243fc76b58e54b00126be19c30e08f93ea0d448147d2100f0ee4383f835e1be8070ea3c22fc78e8d1654ef4acc6d85fba4bd96851da120d6ed096ec9cb4822f2023c6800562e54f73c",
        "b16200e66dae61e3ce920b48ee7afeada6abf665dd76131e59215e7f4fe0c66c8be1d03cb13914f946aa23d0f3074d2fb21a7f7193635ea5c8e7f20519b63d8111a50ef684fd07e00d0ce76a7a10e2854d8728eef1deb60ae748d582a1bf460994978760c93f40e6ada6a1e6d00631122c62a38989a54a562ded035cb82d4b78804f06e9a614b2f5c4591936e2624b64264bf3d1438aad8604bb62facab832d735eb0a311b98322e92b8d473b595b2c64319af63553a5d61d6a6f0e5a616ee3e6de7e941655de05e97a8aff9674effd61b5ac26f4d2fb5e35d4cfcb0e06d79359de311b3866c5fb7fff22d6c6d6382cd22c417f97ae7c76ae2228a91a9b182590ea9e6f28c0f34d3150ca9e5406cbae10c6e8ec3a5b1a0090a48f1fedf2b34fe4d3205218f9a9f512a067ad2e319c1a70aaf8355e746f0874f330cef665d90db3156c6fcb30f35c7900764ea2b47a9163d6ec816c6f9c824dfbd43cd0147b3d7a2da5509d396e994d6874346133d0b8a2ea6ee5cfb3d280e4af83e44d7e6fb2106455c60f2b42c18dd5149acc6cd66c8",
        "a6b10714a7013df7d1bb50a87161e5b0269ed04af029719bcf3533e16b7baea315d0e06c20473badfd17424fe4c4457c8e56a48bf475884a247e4570d2d77ac874631de23ddf0b3fd923428f43bedeb0b50c64aa2b46bfdc3d56baaa5bead7a3a0b665acac23a67f61adc24b5c948b0c4653dff38193bb1a6f5a94a3dd50efb0cf32dee1dba60c29f27d5492b7aac41f7269620b2064fbcb981fbe755a872f85958e732e52b243797c812fe1004847042143920fed91fa761ea9c592d33340dbac4b0c76ec0b1b1eff2ce82276b3dec156ea84937ef8b6f651c4a4de78bc4f453d84e3e2ff0288371e294ad4f86623307087aa7b3c8767e1e06c6203510096e3a50054d92d49eecb6f7253c82daa44f52a6fe88846d43da4f9a3732b452b0b63328c1a28ae819ce8dd1e7c8be35b270e217fada8f3715fdac3da5196235b29027a291b9eed0230378c7eb7fd12d2f3992ec7efc1dd509535562ce18f68ec794f716a65a8ef10f0628f3ac02693e2df5c",
        "8f2abaa571e3bf363c9c4f308f23665b110134a153fd0631ee0d41968fc5ee4b87a4c00d0ee017c0eb446cd72f5df8938a36969710948b9bfc2d209869ad090f90de5197c4271b1cca32b5a7366de38e3e186663873dbcfd419ccf3808914bd4ae724b8acf058c298aa7680beee901cb842109ac3a3b8f5d29844acad4ea920dabcc28666accc1148cfc1778271f093f34c8b7fac564774b5f2464d4729848f265c46aa9f290624dd2dc9bd95fd3c0bd23b0f6e09f7f0ed812d166d47e62a2c720e2c3512c530468c3ae15b53ad7b4432754e819e04bafe7b8f523bb9e4c962a9ccbeb01a46787f7eaac53bf21f42b5b2408f5050ac3bc8308f5154a2a66cd71f1ba80e26c528d84b38d607e8c1e50ff30d8f50a0f2e19fdd4c7d69e7aa05d23e6dbc7f3df7e8f9c61ca51644e2655892bbc0230bdd548cfb378583e7b8823c35f0640210e081ccfe3c125f9d7146a4b",
        "a31e715c12863c386fdaec598dc713bd5c6bf9d88fcdfee5a65d1782810a6b01ca3b95d74f98f80cfd9f68fa4d38736893606d101dd227e55ed0e8579bcd2ce213a4759a3cfa923ba2b1a31c3caef193e1080bc54fb61a8fa1f07a740a933e4bab05bf250552b440f76a57965cb9eb0ba277afa4d7073c08924bd538eaee583744b4b478c798a583c2617d131bedf1fe6e4e9a2946ce12e436140275ad1aa9198d30bd046683d97d6698cdbdbb4201a408dd38c597be129c750cb57a76352bffedab365aea489cf954f997111aea09a84309ca649a95b65df9edd10076d57e0bac0e4418bb68d2c358f17cdcad2ba2296ab7bab3c2536ea37edcd89849a0060096c275bee00cee6aa9ea4f2b13a6e71c715ce0110a89d9099efc039e179f63bf3ad2f47a74d7ff2a05383a577ef18c6e",
    ],
    [
        "b0ce8741fe11d06814d66093f880049f1c7f2da90f6e0ac11ea22bd4549aa02f33e57d1c1d09e86e6a9f14cb10bb287da744b5cc799629f520daab546c667dc606741a7dd22e8db3d5eb655bd7e4cd2b71e4a4e2f41b11712d9762466b36bfbc8d6a99ce85071abc8271a887250061fdf5889c18eab51c8602dda5ceff7eb3c1592cb198b2a3d5c16954f95e4779294869c4b9798864f11539b195c685bfcecb9561c5c7f489ee38d9e81d3462e63c6558705b2d01fe39d4a682a6a586b5d39b539261134ca0083cb113884fdb10f32b720942750b082b78797d626288e5ef9623546dd9696aceca46874b908d08ed8a3c4408d758aa13108a3dbc4e88607452025efb1131c5e7d6cc1105726ac906103eb84d506b40dd60f2fe618620c4c56f5b9ad9445b5f51f92488cff9a6631b741ad69b0729ad306fcbb3359fa99aafe17b8207d25090264ffd4665b84e999f842581d85036e72fc57d37a2cb8ce2e0e616f4c8f3b14eaf42189ec67c2a630c2e594133c3fb49dac0b8641793469114edad25351f9e8824868ac9f646225c4f52555ea32b302ce383f3855fab403ce91f41a10a2cb97fa556675c490afd70547600e85105588c97dc23ca34f517eff19ae6b88069a56c647fc59a4f9d0a69b673",
        "b8f78e14e7a56188677e76d066fb624935bf147b0e4a9d1c51ac0fd433dcaf144d4482df429b4dfd238ed8392526fa5c845a3751395325cbe503e7d54435b2aad20cf9d4979f9e55c2aa41178174d697da144ac1f6621772f2caed8f180f9e8aaa34c7db06c1468d368a22029b32e0140e4315bf1bfd7df1b0222c696b1fd5346d9d4f352a1d3037c6a0a1358b13d077426c440f356960fc15f0a996e04e30785ab84c491320548a137e07f6dc1fb4484f382aee51ac23cf5634df954348077f05ad511926cb4b0535767ad27382c8081289bc23912e4d0a8fdbd0a3846a03634225cb9afa018841309d8b9a1a17061d058801a5adb977c0372335fbc9c200625224b25b01f68ae8dade978b8ab7c74522f308e6be5353e6a04cc01f28ddb040e661a279548f89053bb79e5d506335b6225208fac72d361205183acc1c35745d71c7dd84e89698f85bf7d231746f6b7b595ed6c15a8405b19ebb0b7239078ea719921aaa48485df3a162d10289c6f573325425b3f8fc3912e170140bcc5c7d8368bfdc43a7ff8fa985d2a55dc9a476b30fc65d82979f87948860fd314da4734cbe0dd5dea4ccd32fac1f8318c6743777",
        "8f6c64401033464cd58a8f31d9651f2f13628e0c4e22e144daed92c8d6d86026161d52c2b65511a1bbe63e2b91fbc40a97aa2eea36a13c1966a3878eb489919a873ab5b36428a8d268314be08b5d0e5cd85c4ac2c08b9fa477727cb100f74635a2afa7c82743251f643d3784a59182dead8d88d01447421a33fd704d6add3671d99a219aa3d82a3f5cc073bbb6f47dd86a44de2b81b581fc25403a86f054df9db2c0acc5b9da987c5ab0df21d230b1e523a969d26a9089bde5d9dd4547a799c116ab927ac04c0d0e98a81447d4ac991b65490004048f96885b093c1ccd42f2e2e6aa715ee0e213ebae75a58d5d6aeb1c3be4be945476010126ec3f7d6fc80eded75c87f10d4f8a92acc661461601fed926dd2bc8d47f7d4a302fec0c20a9b97d5f67aa0f07d98913e1b9f8ca312f0d94719bf7f05020bb66af44f7dbd3a2b4a963eb2c4db03a6c4d046398997d82f5b723926ed1f9fb513db53927610d9334c4f060e1a34991dc81d8327f76e24bc1c73668efe65ade77d333ca901b03072d6a5fe673ee78316b9b27fcc443b7ec425e",
        "a6890def6b6d70e1361ed9ffdfcf77177963b445ab40c2a515b560e3884761c43629405b0f3e97c7b70af58ecd2a44aeac39946e4c077fc593ebd83fc2103c4591d5907c0571ea33e82228a44df4ded531141603e1d790d7bba8c73c7893aca1a33d33df89e1386e3ca3d7d467f72da9c2f6395122067b8d9ec01941901c04b17fb55bdf5378ad7eec9a06c11fd7903062d0ff76a3ff66f335daa4508db64466a71cc623023f13bff9e99948163f8eb11fd04c54e62709eb78087019584d3be00455a8d9c9d1abd77a7182dfa21d68446e430c1b889d01f7624ceabfe82206be600cfc6740d5892eddce6eae99eabd050b5a693c195ac17eb450d1c4d808746a6135e4e89ebdfe5e79dcaf92dcfeb0e0202df99e296229c7647d2c471a3c4250ed014a9702617ea13279a4303fd481165b855e5dfbd13053fa66a1fc9688415494e909048774a7cea3c138f523758059152493f7e61a728bdbfc48ef41eeb939fc9298e36239c42c3ea275ad645d83ca",
        "af707d724df04f49bce2eebb70dc9e4e7de272e73c5aa00dca614a41b2da2b1c0912c457619fed19cfce7fde86fb7609a1b437d51d3da255dd90e688456a00094c52ab5e63a009da8407713c3bfb38a866353446ee44132b3544781b98b4dba7881c9a15ead956bfc9bf97a71037edfb70b8f6bde431dbd8c86e0c468a72b0c91e84d39f81dafbcd9ab46003b25a746a1f612aaa79ec3f2d01e72bd789c2e3ffd7a6a26422c03224be137bcee526c90b414d424c682497e9dfa78f147048d94124ba3598c43431bca475910e733d145c68d9db84f7deacf2aa21fe7ed230cc4e68a5abdb02d1755adaa6d14dfb58d87520cc1e36920c33698e1736690a270541e6c15879e3b5a72f079c17ceddad4a494af252cca97057798d0fddd6a2c098a4863220782607a23fc3b1e04e4d96db7637117b03caa2c199deb3c2120288fe3ef34dc6b498337e7ba212a7437c204ee5",
        "96aea7de925ba5977aafcda54b5388f5cd5fc36b4f6fb71eefebbbb71093e2b029b16a81bd330ecb0ed544d50d53db19930ab614a353f9692b56aac8a277bcd521a66996c32d16fbb36abdb859b075ce07bb30cfb3f8921ed01b8bf5838fb164a094f37e2a064bdff3535c53c6c18dfc4d48d6588c8b5754a00cda789d8e1216defedd665b54aae316ff91e322fddf441be5598185dc2fcdf688b3aecefec071cdacf1be7bc69b32c299812b3b360a3a05d1d19e64ce3368f274e24b420712773df71fff2e28803ba3f9a96c82d6538e2428eda7b56ffb1a1a7b4462757aa036bf736bd716a04cfa9270fbfcb790507b4d20886d4e7f04a719ca50070fe855a187beb0d17e1a18aafdfa85de06fe3a0415fd915ef7f7098ea5093773ef8c6eb512a99057e4502fc4af76f48e61fac331"
    ],
    [
        "85aa19bca1a5a854d220834b91d62180bbd59494b41c6e6773803de0c7927fb84acddb1e9ab1baff17ebfeeef349eed8b6d62e8e2f96329e123478805f727385655ab4ef45261da36551f8f7e60ec4779cc86d9e3c760b1bf2137d62e1df6bad95bd6ffe3ecad090cf0cfa31e76b6a6ad16d4a5481c71266104c148dee5935951dd0750384f068e88fb804f303bf27090b05b27d34f77457e787c070687db074a8e6b959dd1196497097c1e819d613f561ce3c11684e15b0fab8b8a6d472238c4ea66296e8fe64a9c3d46db0f585ef513927c44e2302547b3585c448d6e70c1c9df07c961731eecdd9c2a7fb8b62f43114847d09825d1214d8720c4000380bc1413bd1be6ea7f3e5ea2a786e7a528dec3998161a2597c095a80e384943064de555c432c97ba88d93488c369a10f0a5a40670cc0450d97106d80d60d40503840a783deb8676e2bf8599c41c6fd2b85a4428a67a56203f076a2864193a514180ba513c3efbe9fad648b1d57898fe7862363b5e76ed64a17e9b03fd1214b3d59d6d37a5ff5320a3c3e2fb363999b94188511cf508cc2fa1ae38efa619418ef3b028f48fa8dcd95d3edf2a9790fb0d6726686bf2cb8f2eb5d8004e96f4abe3bae50eb6aeb5e8b0593453d0789bcb5dfc736e",
        "80cf87fece8c172c8c0a0324b934ce7c1e0b839fa706e885eee18a28be08d80a6c2ae7e87b3d62d90bdd54a5eda47f0b8ec8a2f482de491810a9e7dcfd595229bba896b564c0454a53b3488a89960dd43703195b5ccaae82d241ae2a3ee21e4ea638687d075180e74787a520d3a6136aa3a89ef257e590a576d11aa14e98894ed883095b5757407ca049567483ba721010e94f2dad2271ca3ddea1260bd98ecaa8209a099cefd441ecc623f0c145c65438923cf98671d3fbaf446a202d4a7683ae79048510cb8e412e3e8564a20a139f07ecbe65c30c783a2dd8f3005adefbd4b7900e56ddda09e6710aafea7538108314bcc3be774c6806a2d148555f81e5acbe641ab0cea9f7a70e7e7eeeb52f6e5a2f46a18121d2a9debd4d9001623902c7824ea9b750d710622d9b4e795b6008da1c138d4ca193958f6d96ee3f65e07619566d2a60443d6395318d83e856aef9b611eb0d690492491f70a82517c98fab829ca30beb484995f22f59ff829ed2fdd86d4404d26970adc14200ee207bc7749f20519970183a586466a536a2fe28236265029883e06f939e34744680ebcfcb15fdf0a6fd397b5a88b3ea831963b1d811",
        "8c0bd0388bd35a681d7d8d8c399f8ed5554dfe4ea18ac99b39a55c86b5c5471e0b5bd05acfe1ea14df73c9e6f60587e3a51ec133283922bbd609e84a62617e85ba49d63d60e099588d6c04bd1939d28ee081f892217593bea77fd71020c191358295bf13c0a441679e4e456538ecf53b0d625206c1941944953797f64e227bd7081daf07437ca63493344b8fe63620432cda1834eb5ff990f618cd133ba3a9a26a6aae110161acc4ac4e11af8825669611ead033ab37994e1f16cdbebdf861e1bf39a84d2a9ddc8858e7a65b392d061d39568ab624d9e70236c573a0e2813f9ea704ab60eaedf295b96c6f6b2d0c42db229c0ee0349ddc31754fbe559d187e657beca2c77c454f097d42ad2e85dfcf36445ca1cb1b3b0bfa3ddbc2aa1a97ba403cb7f6e76c235ca8e4c88dfead37e61009bb8d67180c92b1daead7a7850236077b434b84c7a6a10920712b0b9fb122a4379845adf1ef673c2e96fdc8cc3a1873fc557f83210dab5ec31be36d51bbf21249effe6c39542014d8e732d850752b0f5ffff62920de6c530e0d5ce98653a879",
        "b0db06478cc010d809872e0258f9ed5ba40984b32d03176257efd153b53dd05bcc79cd244e4463bd0f9be1d0d6315987af759ab4f26aa0f3dbea68c57d43642600d132258053eaf437331b079f8c2e2d1705bb3dbfe12d045fe8fb8adb1b2a4cb2eea51f3f1fb9f1bccb8b48160b13b9312bc26b3b3f28f2448dcee45b9b4a567528f18389be802e56b8f0fa06ccb20408688c0a00bd529bb05863896e77628789738d98bd95f5b6b74d848a25e45379662def7afec4ad5b18a39936ee1d7d9c574ab44cb5be2dcf15255c2fd35fcfcf0a96eb561d11dcbdede6f7a17fa3b576d74def27bb90ab1412c49738b066910363178c241f525d3b04da4e4f6c5a5cee93d0c5ece477ac989649f49975e88c5f5b27f7d13f533c297557c97abf5268f595161260d49065d4a4ce5b5a464906b117b6980bb46a9289416a729cac3f20eeb39f6e0d13b8a59b3b3f2592169f1f84290faaeb07ef2282910928ba96f2a1d381c91756c4159e92ec220ab2ff1b5f13",
        "8a9fd9b593a0d27363182324734dd792f727a3699f95fa6d6b468e2fafcd9e8bc78cb1872e51ebe43e726f57797a4639a41196281babc892c8ed200cc9754119ba23cb2ac5699f3937b96184157b99d3a57154c730575502fff460c25a85ea3c85da7d32c23a821f87d2189950b9817716bd2e65ad8eb2d9563082dbe8b4f799b37a030280b573bed8f8595d2c81c97369a8fe3177e9b83369b97a1fc87d6e1f0f6841f46bbe5b8555ec93903022f7d75eab48b5eb092fe0437d5c2332b7ac18d9e958a1908b99485fe75b7a11d5e97b0fa9861a73fe4c78e26a2bdf78452b770e66955b3e40d118ed33d4e5046bcbf0027c836d3624b1ece574f3bbc263964dd32cacd2b42cec47ab9b53eb16e8d1020f847c5882e76400a7e40547c77574b0548a06d6072b85e59459118c74a4d45d517b83150c0281b6ed4a956701188b8fa4ec86370146fcc28851eb1c75e7d04f",
        "9334f75243aac596a84a09b5e9dfc534e3f407a325d9f4ac3a5f34a057ac6edf547254feb7ab9bde77305d2863e56bf0845e92dac40698e138eaf5d5e0d399fe469d826ab79ff9219b51c8ff917b0a9d7f5ac19f0e8ed5823fc16cca182610d5a467bcee582d0094cc7c4570c9a254f2d07dd116214b591ff1ea0411365e5f03747abbbc8b25dfa3e9f459e8984881690cce65f8bf765983c8f2c895e3c35b1a727da964c831a162b3841e6fc5dc780a6ec1189b8f1b8ea6191ce23219a2ed6527c63716f95884931d9bf8ea6a4abb0040bed9a0bdafe2d99e1e40295471b744b21a156d4febf144e9557dce94fd7fbd0c31405dc45ce9bc531ce7bd964ec7a945daaf9f04a46271cc4482576035ab5960b498def2de82c4b28976416e998468de5b8838432ca634f6b7a3d432bf688b"
    ],
    [
        "adfe4d742a94d1d7042cae56a168b0c8fb743928cfac99223d2dcd3b4598d09f40b47dea73651f9ee2660ff6427645c99060f8ddfbce314ae4a97465a246ffdf5e37eca67ded37740d4041baab1bb646d90c65acc516540b9126556c9bdd76888c6fb8d6b4908190867b65c561f6410eeeaf89140430c5343dcdac19c5441e42cb92f9aa097389db240070a89d78533430404fc5e7dec536a13e56db29d56a3a4a78810d9527ebb6afb47fb71bc428b3390be914d2e9c30e52714e7af08305a03fa936c1291181884fe5d66ff81a952c049120fb3b3737177115fcd56318e26042b260b5bb591e5af9f35de3b5637d18437f0f86b17a1c6b54491c4121cb7c9421f91d17325883e3cfa43f49722a39534fc1e92df08c6c2f60e76b1a0f36f0cf4e19026e593ff86413ce371f42ba863e26d31191771872f0ab3fecccdd5f602852718aa74cd9853d29fed737aaa67dec4208aa8ceae30b4e833af7365912c81064f98d476d9d7632d850e21bcc5a719630d7949935d0157290f30b8897724761a3c023963a1a96737d467247e38c59b836bf5091a7bc2d511d7e74df13010ce1938556fc27da05707e26bf9242fca7934edc702dbc5be2e28e5e9b11a352f28c2ab47dc991831d27abfe25451310f8cf",
        "b1dc361e076a12d9b2ac4c05dac063c64ee0a2eda403be764f917ced1b8cf04adf249eb3a3358aa80e4f0b4db19da38ca3cae60abec8ce299d7e79ca8bfaece4491954c75f461f116eea5429c54fd7c49c7560e4eb9ff2d97fddd503dedd3edb834283f94205f21dffd8a733bf5dbeea78bffb3d72b2eb3db540c33df0acbbe3808b85bc749784e7de6006ffb089e6e44a1f30e2ba57d28f26a66ba498f3bbbb743ab928141132f9b74b1222dfd29dcf2b03a4177bb488e3c47e92e77f2e3f0902825cec522add96b6a83d4f98b8dda6516a8f52f5ab15f165f165460a8aa42993e8062638fd2e26c88614f5175e856514477a298bc50e8764556183c452da0ddb772552b2e2ac02f9518dfcee14d28a194ba378421262a9e7a690ceec9387024c6dd9bec230baf3bf5036c6cd28395f402f9fa47d12b9717339fe959f9e46cf3316054c500f314fe2235b3d25c2276d3e25d9426ec6c34125926bf6c042a7ea31bf6d140fce5faf6d546d53245a6bb5469a473c50208a9369264781c5d1b02601494f0cb4a05ffae549748b51e3a9a444008ed713345bf66940610b499d176dafe17d603b57df67c3dde370157c040c",
        "afb1ceb4255268af04b84e4a7993b26ffad5ef4dca6bdf98ee09e2abb5f3531680f701601fee28a95d9aabbb51b1af768612db4a8aeb28cb478cfbc0781d70b7bd19fc0a15c35c86fee74553526e2b289c5a222845e32bc472faeeea2ee743f1b8d1a402278b9509fd9fb8f788a450a8c57c71b1696b43862ade9aac337f506175709080445651e6b2be2d7f9ac496e96dddd0a90702c805d1d78e0adaa19ea9f24e5ac8651012f1e7fc7e308cba65c84cc506cd0223dc98e4bb5df118cc5d99d1ca4aec56fd634bbe485ee5888640ab24bf4a77a90095430626040d994019d4734e2db1f9afb758a3306f929e8576eb04199185cacbfad3564f1e93dfc8392de4a86e21a2085c0e22fe9c516e1252db466f120a6d02457001ca89ff1b6c5645b5b19c4c8e3b89d5db79cbe2866b5202523fcf7f04634faa9ad10ba22a5a14bff1415ac26f9b9f5d5979ce51c03769bc4dbd28b75c6a6cf9e17e0ea02bc15fbb3a9cd6fa215841d1e8abda12a34814d32358facf87402ec12ae7af61ffc97e0943fa765c2398fccd882a52eb5ccc47b3",
        "ae84484575b3f28e37b2a47488c1e3a4a9e3cfec856e77d75ad52b5daabe0cd7703dd38f115729ece76c9037bd28f6ce80c7ebf9f99ba5e9f4f21e8ad4a307830eae97968477405eea26019f02442614c16ec8acb11cea419cd3af3c833f376b88d684e487897544a5dd79b777e72781dbe6f80e7f915f5e09a795373a9ec32d88b459e8ac1c6ab9cfd543b6eb094f666c8d98393cfa95bbd94831af12d855e196825ad47109489361c2e99680de06d36d4147d9d4b3781bb97ab87be0130bebefdc38f5411e6dc9d66e3d77b95a2f881bfb80b2ccea6e818217d2c9e100444ee1d53062a601907868abbae1d546d97a0a7a9a0788efd7f246c55796b94db3fde669ce15eb02e39b5bc275cb8bcb8b1365269e7873d9c49af1dca8d53131dce7cb14ea21eb8eba7ecbb8e317d98acd8a22dab8be36b7e3065b381833fed9a4ffa070ae8cdda52d9d1091a6132ff41416643179671dfe848dfe6a987ade4472fb846d1d07807bc8e493491aa0c87c4b1d",
        "b0349feb877b404c8e696ba31d0a6dbf407bb8953e48903e08c86ef5e8b3750f0079494657ec14c0c31b2199d45826b9b25bb9d7ae400a5cd4c11977231309ef5b121b7708dda2555f45f7c2e4fb422119f73afd1893bc1cccf7423dcad1b6fdaafb0dced232b7f6f7860dafb4622d1803daf5cd5e7f0216571f6be5de7dda3a203239da8447ca1ad0fdea57e48dbcc53a91f4d983c4310b9961261e2868d25fb3f33d2f96b7406853f22d1d7a8aa87d53484b983311f668f1ea1b200901b568c6d6177d8a848b1f0f1415bbbe6bc0a906f550d86da4b6e056cc9821756d8d6b1323f7ae21bd7c1d3ab74bcf52e24d0b67346d11167a80dfea869ec38ef612137524c5efb8162b8d21f624af5994cda56895aa8b87979d11829ed2da10868b2217a21b51f9bd1a5f14385f56df1a873c4f285c6fca32554ee6fced901259c5955cfe625ab06271b7e68b51e0cced420d",
        "874608ee1ee9f3e27b16d4f102c994a9e39858dc1e4aefb66d4741e680d25eacf3b589a559187a2fa02584912ba14248a38ac67daa2fb87bac042f52779bc3e39cb6896bf0ca818a448a9a58f176ca7fc7ea078682ec577f3cc9b151b33a754eb5219d4bda861ce281970410ee97985afdd475f68fa3e83bcce06f3f0471d51a73b8a33d3bf6b096886b2e54d37f74e3223f977593e5167dfad675d9fa27f51a51a87027eed55c2effd4ffbb00f0f5b6222eeb116d86e2de45268fc24640e110a604b0a58408da3b1e948dcff2629b555ee7d3af2675def9e149c22cf29902bb9721d8b76038048f8c6368e09ae8959e6327b26f3f59a69c95095ec22abc03eb8a65dfe6d15af640bfe31faf26eefc1670346d07b3f6f97f74059bbe34c1cea76300bdbc88fafde8f92dbc9b41c11e00"
    ],
    [
        "82bdb616a497895f7779c9abc64180bb30e813498f33986ed7d88df46bda7ff562fd95aae79bab27b8e80a372bdd6efd83fbc212acc03f274944bc761f7c0b1fbe000fdd65a459868b73d07d9992fd81c2854e5c48ed69281b11fdc2d6cfe09ea963c421772bdc31c1c83130470bc49560eff463de4015001c662bcaaf036721698d7024fa2afa8ccb428a991f21ea9e262ab05e828ca458fae5f7d2a58e885137709afd5a379af545f22c594b60b22f5b593bec490354b2a7cb53ee41c0a0e669f3c2b593f79d49298e5baca036889b6763c288b175c3ec799021aaad912136eaf29d8f49fe272962ac30262d1d38e66ae4995c8ea008c6f1b5f7256f8f375f10f08629489694a3dc3e63ac1491eeb13b5afe2492a8e5cd42ade13e6b794240b92aac5f089e1c4650066d7cb28491c53dbcf5048c2587c2dd688d9e1e9b32a5e4af81e7d3570f096381da6a6d4d4ed232c4a5fe15abed118b16fc259557d82f9583aab4d382f6afe7f0ccb1524ce0b75d6e9bbaaff34595092e5432b8e34d21601c0711a81c3f6f9b0becf18ad8bf186c68e6d788f99ed0153f5ae8ab955f327774006e0305493801d8f98f4b84550358196ac315b186f80e91263019836e7eb42b3f59a73aac277db783146c845118",
        "8a07ff385c60674abe9341b69d75865614c34f671bf94e583e0ef221c070782996efc4d159c270aec9bb3e3c97ea026d8660a53133b67b886fd645c80f1647d23818edc67b3fe51aa05332298b0b25a8acba6b732b1ea2aa5e20a599e3ea05898b52cfeba21cfdeda59bb67970d1b6a6ec231abde77aeacaa7a1d87183dbd580213cce39031b89698cd40fd65e4408d76eb2bcb59c86fdf295798f179b36e18a35a1a570e9ffd55b287b88f17175f1435e7cf044f61024b70fafe3368fbfe9382562568b0ab4275cf36f4c80345c9126082e7cc1b4b0700be15217290db3f0ec5db8fc15f80e3d7eebb40291d22742f14207c260dc9d5660edf18c2232d3516a73f950f8934a0c4cd3adf26fb766310e652304416d1d0ec3cdc98eeb015b53dadab93e1ac5f6453e626c3af711aadc626d075739c3f1b2709cc61c7e689848f0aa84a6eaf5f50960626dfa097f299e3228f4bb1fdf8f3a665b10897652dfc0970b08bbf6fd948af2f68ce6b1b9a1988218c7729f471a6a4f5a9336ec9ef4d07c06964127cd6f961cd3252c2940a64840209b917b08440915605a4111d94511852ce9c7344801aef355b83af4a2cff53f",
        "b9f2d2d5b21839f72a78a1e7306e978c34f8a3ce226bbf01d2f7cf070cb788532bec58f638318ca15602cd87427647acab11281a939f8ae64bf529df619f1e48d97130a52cdf4d065e68c99cdaa65274116c9875b10a96953ae908f23e0a83beb67621e728d8084beb9619f963375c6728f9b0f2ed4b555e1bc09ba4e123e8e4250f15119309442daa20f6993a5b87c208270df8d5d0eabcddbdb7c12046fe496ee9359464b7d378fe037646e6f68a82095dad58c6dbc67dea77bf9e50f76c69dabf9049dc465aff3ed24780a119f7ae39878862e966ae5a725e7ba0b12690f3206f2907a7bbea31a5d492062aed34c47207e982ddb73d00ff2ad6575d3343f904c8a8c93dcfb2a36044023209188cdd3568c587f067e4f630d996d2de2b82cb0c3705a7292dd5460dd68ab8f977d46c1b3297f417f58bf21e23522a8e12a61f3788a01c5cf1592b578a9b23a8b622b6185cc6e4f7978fdc44cbd27eb944432295ac8b2a0398d26acabafba99f3d114d130364d9d47feab1ff11299fab927090150a68451d59eb5bd667d7c3375b13b9",
        "85fb843f6a85aeeaceba0b2e9ef8c08cac156ad37c75e8b2fa90f18423982dc757223a3a9faa1de700843286895f3111946d2ed25601d7ba04edf0e16384987d7fe0f09506fb871a414b0d49befbc16363bd1ed6663dc44287390db7040d66ed933f024a1939b4e3740f1f3ff96f3a4d032dda24ebb6d3aa1c6a549579f367d355f3d0b5e4a7d0509d2642fd547e5a7b5cd7a95ab3eff1c0d8bfee2a08d7ff236951e8c3c28aebbd246edc17497b66fc1e966271736a28635de884823cb2caf13644d9f03d3b7be2e6874136dc18ae5659b7538239d4cc0f40576c9daff8d2741371e550952e33bde80883575de7a01a4beb857f0d188316799d1015b1862942d650fe22a4b0ae6caaac8b11c4281ff46b31100a783cf1bd5ed38a452341e0e4fcf8fc39e0ed9555b448967cc7028b4a211f5aaa101d915cb84cdc348ae43b8990caa1fae289c64752f05a009fe2998a1780b9dfb3d6f808838dd1d957f89d1fdc1c25ab6df02c4e943d7d23ea0c3174",
        "98be1f85d195f13dabe2847004fc6a3554b8a8f65878f1930875c0707e5f564bc0c943ae828d0ab668521851bd73e5d7afc2d54e58ea6675ee64b892e1172439431c8a121759b85680ce4a83d391730237059cba5968d8c7f59f87a630eb4acc8b53cfc02a61e3f8175804aba62c36a48572f3658a60732df05936a37a6dcfff6e7538d163ffab46f10167a4e68848857366f4d507e103cbb3e7570a2f1a58142e48bed661aadf00ab64c7784e7bbf84677fd4569f1e02d134ece4c2e1444c71440fb00ce9978a8548a898c1af348f6a0f14855594757cf251f1bc07b6f33907121da784f76321f12bd1bb173a49e200452277fccb85a445b6ef9e8796006678142846631ac3419064f38cc5620c7ea105f2ceb2748dbced9dd815cd02260c228738d7a4ba0ff85c8de21eb567e4c681521f07dc74c990f1e0b5691d3ea920a02d7a11bd32709e616a3422fdb8dfe4c0",
        "a3ecd77ca11e2c875ccac010295d74be8fa71dbbf956279fca4d813c64df8756e608d299ffb17ed5472cd17aca9081e783f4ede52ef5dc1e82965b501d1b914d98d2ad0c545643402f6c7a9b9684ab61ba4bc05b198058dca4cdd6193bddd5e7b0dd827f6fb31d812666752a0848e89c21f7bea087f681266d2ad15d89f42b19aba4cae5d17afd0be2916ccecae7269e71794d2ab4f22ae7a7a10f070987eb5808f0d0dbf9842e8f00e13551f9d4cf89472194f60ce058f03959cd6ef7fdb86d5349506cc0bb8b6964b383dd038bf18f4adcd5cc019268dc9320207c8e17cb45f2ce6086d2950d3dffe8b8030018c0711d95695920232073fc810c349181272f4203fd5e4401222a8841b861b36775042edbadd623d73e370c84820200b135e0b7b8801e89bb695753bce7a3edd16517"
    ],
    [
        "8567c544947b6e40b5110fd5519508432b8af8e4783b6aa704058cab7cc7c3e7bf35a868ea57795fb698acd5a506c4dda92b492a68c765ca34b6b55e0b0a99637ee34e81ebf034b6654300f0884cd982cda47c185d2c617bea3f736ae45b26c49404867ec0f538490978f29419ad9d9cb75b8f1bb9ae249db08a7b7d66d69b9b30d7ed5fc51f856d62ca003ebb99ccfd5b2d979bc1aacb964a4d7058694eefea8054c47dfb94090d3f02f50ee5ceea386dd68072ae015e2167dc2f1e75cebe2aa579bfcac681961df6e1bb2fb494da49173848d10ac599d07c50a933916a187a7766b7cbfd912b5c54e12bb0884223fa10afd33e2c7b6eccecd5abd87715a863215e57ff87b5a7b9a2c9865862f5c8ed4c273ef0a53dfd213553e176ab5ec1eb23578d3c1f43d2bf8e124782d97f8d184d2bf40adbe320a4dbc6c5082e5d3aeec30233fe94252270a44fd41aa99c4ee07344101a2174a76896eafae1f9b31b4d8720d239cf9c40215732426c772a67361a6d8fb3a6270a8f3ef6fa584e676073520e08b68f9b6720a196d538d27bcc055fbaa57c4ed3e24e51c3e25689039030e291511192b9b2b65e9e4f653bc528601cea1546dfe65c4f91ff9381f2b97b71c6ac0b2bde6b081a4dcbddd366dbb6f5",
        "963877b6cf2ad5bd4979fd38c779ec28c43b3405ca776436ed49ebbad228a4723b50b18bf813b2f923cd5f65daa71a1c86649f69557cba9383d95062587022fcec080ee4412e72ee0e8a5e1824c11807952e4f62261450d962a6e64df33e2212a76b8d01b112dda93f05213bf968ebe67d4d5fcdc002bb79fabdf2ea10f17ade8b036b1751b1d48a5599dcc0d9d6924b2bc31e09bb77415eec809cba6e1e6375143b84a97b4079256b9f20716d530b7b2d482ccdbff97cfe60e746125074e1992345b75067045cff902bb623677d11695816bce5b502f85d9c786ca2b3d855e8e423995e141e4220180799bc431113984048bb066e28a6f4726b133f8b54eec42804985b8014718b39706a86d15cc2b3289cd0cce02361857b7f3a5a654af056671dd315b8b2d53719c8927e15dddd9c1edfe666b6d2f258419a1fc4d89a24aa270f41585d21498ef79fe04c941a2d3d00ad04861dae9c2c5b02efcaf767613d4c9379f6fda7c5f9785a38c7097a78ea450bee209087781711eb384b33bfbf1f1da852dab0d606be78e17ae6d5a5bc776bd0f75cf5737c0e9cfb308d62d9ab0d548932e491a3f1a6bbafbcf7df668559",
        "97e774925278941d9748f1d824620d6dc16bbd4f1722049c0420bcd3fcf3d42d0c6e00918533fd2b38feec3572e0562ea2c2fa780f98681b3de857625e1eac71a186e2fd36d0e8aff1ddd2124b18e2cad18d6eb0ad3ce7ee2b24ad8bf40cbfbd856985188ad25110acb352502088c34a06295c87ec59db0cb29092222550a622d7e25b2d41f07db99f6bd0c7fb4242321c5e4be147a449eb5558e8144609d0fb73a082516b22542909cbaee48109ccd416119e6230cb3813216f0fc738d33055dd7044fa74b5bd457c5654af90f46d6a4e539f15dcf703a76a440a7e27c567810765a315c0faa7116c4027883809c1c249c0eb6e86d3b251d46f778703ca60084b4bdeb01c5afab125da5d0e2ed0e3a038c5fecef187893ba9c94c8749eece889ab41f47bc7bf98a3ffe55e5de7e37d926adc19e39d2e9c53fdd1a79fc1e4829d3e6fef8fe6d5de76a5aaaf895ec5112709d429014903133a1e6d8f3b20a6d04ebcbc12d204a7f173f95333e235e80610f9a53f34209cca73b02ace17af617e06edcaf228c48c82db6d4082d40787710",
        "a3d12b550bb620d6dbf78284a926b1ea1424c639ecf44bb51cb4715be051a5999eee7c9f2e163202ccd44d08598f0d398b0e248d061fdbdadb5fb321ac288539f6026f03d39dd0e69e56fecf859df6c79cbcdcd258a81639079e1ab9683101d392990668bf31f156b3d2b96d3079351d4b9fd27839b383f694a5d91f2c4cfbb675fd878d55350600e27dbf97cb506cda0a330e3d29f54c96b2540e80b9baaf69f12d490428d8f4d7814b5f7617fabcb673b19460d62fc0be35a6c76a56857d6c61d2c6c02a88b42c1c80e22a791f4de85ea577c2cba11ec5d9bb0de3214ba015f90565ffd3e738fd82159b0fbed9f75b2c8d5191af0f463684addec3b57edee2b3c105c7fda8581c470e5ca88fefdaae679f66fd14aa8b0209fe839fde02bc44c193f4fa0c015e0bb4c3c0a8bbb8719362ac85984f45d37bbe7d07d90f39131440bf196aca1c5de6acc0cda45b3077a42c055bf692ba471aa68a6b361375e1a0020127b2b19f988a1448a95a9c1c4b65",
        "a947388509fe550002ca485435ff578762c24f7ac6e47f62206e3ec5f67bb06f37f0f8f36976eaa1b94eca0c7491bd6e8cd92f6eb3bfbe8c8a0710c284876d1654d22a2fa46f6b8ea58714a776cb6edd4c87725d903ee1de1bdc85b7cd56ab05a7223fba32039dc3d487a557d0fc732b88f0fda091ae81a58cd0d9884498010cf254d4a558b2ad66b1363423e24721683984852945a40bd8b99e845be9cacc54ceca222e526ab7ca8d1a4991af5560b83a614eabe17967e0b5bebf1ac9d452c1f326192d259a8a78de769b61671bd7c064bd4b18a5670a55bdb5946b86db501e78bd3a88339cc2d669e3bf76553a83f9325ebd87f54340b904416038ff2ee6b77354f81104945752e6bb6a861e5e4a7e06c5dba5a43a3265115fc471d1c3e061f784c916c357f9069ffcc78867aa7c49672d971524c33e69364ba8b37f572f4e35941e9f29a262068ebca3254cf3646f",
        "a6c412ae8eb1b85bc5181f61dee64d28092b8f4bffff8813a60168d9c9b74e640f16ee5a1680db7d796046136d5ea98292b404be4d02fdb8789b99fa1067ae18baffe51b24b911f1684ef513e5949d86eb830eea072ee8de433f12f3fe114d6bb5a52d432d06b59c8e5186a6cd7e72627189158210d302b7dad82cfd4232769242ccae607056e43d0b1a57dc60b0069a0f52e38c6b04e0f98bf2c163fb9f7bde19f12069b13abd826b642929191a76f52c3fa72a772cc98ac828875b261d7f93b7981efb14efbeefd0efc50b3134b8ec52f31a4e5f02ac709bed3f1ead04c401a77a5cc31ff0a3ff83ff0d5fc2f6353b25833237fe1284171eaa7e7eb8103ceeb100285ca1ec554bdf184f9ead89726f410dd9929c1bc505e35d6e1378f067eb177e4fc93676ff0e0c73edba5caa21cb"
    ],
    [
        "b57206a3a482892f7f25f8f31e27881ac0a5c4562a88233fc94c30f770c5e663035fa229d9775eff9544bac73cde319e9384bb29714abbf425058f9cfc73f0ea55be3b7a26074f99c8cb44f1004f2e81cd2ff39a3a80bd5ebf4c83018145009994b2be33db072a57428c12b868d683fab118d095955a7bcc4e557e38b26a6f9b9ff7bf7f6db26cbeb7a3b4383797f5100d214a5ed6bca9ac63546ca902c239a4493bc0c0ac886af6eb162c1508c244ed11b3da8ba8e8392303a7f0ec4da4725f9e02ba33ed2c5d19f82089dd56e746c412d0e360c7ba57bc8fb1a910e5f7f48f3e7976735d4ed3f3a8d1b84e841c170c6eaa4b6cba9cf668a792efce1a28a57d8bfd0f832c66489c19c43434a466c163388a3a7e2a8cc712a9351392f6114aabdc6991d3903832389931cf71127358054264f8bfa8925645ca21c5299a652413b355a9ee91232413f3e029240bd135b441b8118008b5544984ddeba50e56b115529e849796cbb880e056605b85905805281e25629c498b3f735f6f554c84ecaa61a97ff3668bb3b8b588b80dc358b0cd0549680961ce540ba20c9c8aa33ddb9d6ce7a9bb9a19eabc41cf1a4759aa83fc062f0fea8639337c1a75417802bdff201c01630e92dfa779151aedaade05cca8",
        "ac79c326da8055776583cf1bd292c76593a43393025f7acfcf7813861e93fc17124b17616e3d7139c900d190fb20bcc8b935aba9d14140c5c1e408467eb1659f6d1184ca87930d28147ca02354d5012f3157f4cef9aa5a89b7f996e94960c454b7715b0a78c127e0443499302108be5764b35c87418c65a27ab27d0151995eb2dfbd9d5b19ba638c563504b34120793d600a0c68b750db07c3185a2bd8c56ce712a0677e149f57506084ceb325ff8fd41b963f191fd51267f3ff50ac33ffb19715bb961d9188715dd8f119da8a2240db53869fd169e378512099d82ed6cc96045068df97a6c8acf5da4df880a63d4ddb2825402274f5d1278693f57e9bcf73df54a5e9566d5d4c1c21bc98f32f3ba19617635a03997ea78e85dfc15a2376d844ed57fceced6c87c117bf342a7cfe145f2b67cb610c4d2f408d5efc5f278374c78d44961a1a62173e54e0fc77cdf46cf96ababdee5c2940d21e68730ab1765cde4628f98152189a8b1d8702b40d7736bd4853508cc42ec1811863cc680a45c43dc2e8778b59dc2dfdcdc7e2d8744940043c39933a4ac4d63cefcc36fdd9cba7aad754f367653cd6cf7b9acb4d9bc5477a",
        "870e1e8d3debaf6322169740cd0a54211be5df7571262e75cd650a10f3cc77904ecb3d6ec4a13113949489ace9d53e3b895ca26fb6fe0e7b68a91ff4d880d79ebb94f0f727e7e525a70de5d6c3c320d8539bf0e7eb7ac32568b0da9433fcd89081002fe82f407b1a25f9b38f101e5b4678a155aa0f8b03470e28c636aceed8c07d8e9b5215ecf87ff4cf5e8e27f0181d05ebff816e872ade5aadc8c1fd0fd820434ebf3b0b0b781ccbf37b79295c43381f8c3015a4b016530fc07eadb845d4e48c5a8054fd12f15619de5cca65d5025b6de00379c4c37cbafac9fdd53962ddfa6d9773cba8360bfdd7daae2a5f19a9094e989ab7072dfbe90cfc59f8cfb3a75f8627e69846cfc9080134d5c8e2bebf7a5bc8cca9cf03e894c05817579c7885000cddaabe98f14c4ff44cf584301aefe6196041b1d3ede222f0fcb3448587e942f457c54fdbbefc066d3502c7e7d35bc073dbecc65f6795386c2ddf20312514261218f7648c72a421d7567c3dd30fc6085317b789d11f30327c9c298727f1d57ea9964a12ce6d51dc59032cbae8b8a5a2",
        "930db3c896ad19818d5a4159ae50a02264833c004a3f43160ada3ea7fb7e9809703014383b22e3c56b78d0d09d1a94ce822cbc7f9e8d1359dcd698ed42e442de066f7cf0fd62624ad81589d8b0be14cc63c5c9b82ac92b594d2b4d8da63164a2a2e0dad0803892a83c43053c17c7ae27ad721d55ac8f7563ba34a1d3100f210bae7b2d65e3ae45ea4d60ab0c5d9c4f013c7fcd6c988054e0d64765c586dc432c4aa444d3cbbda369dee61809da285ca36bacd27c341f3cf34f104eb757c06f57192150f8e5871a4e6c7cf1261b09079706e67bd837abf9fce0fc497bd69be58eb00329158423f9ad772d73cb618522b345e8fa85a02edeaf9104f9f3b9767da0011682a657f40d9aca8446737a0e835500c3d15809e618ba8d021a701f382a3397f6188efd29bb4e5adebc351a155f3f35e1a11e4a07893d933e2ac8e55b3e2015af5a276ae039e91cb64f39021b8159050268869e14fd0c823b7ab7266a011bcfc679997c1dedcdcb86f49433a2d631",
        "a1f52e241bbfde0c2c968409449689f554286d6b504c56f7eb82f55d64723501fdaf3d3406efacd6306aceaec5511b92a0bf6de3a931d5b4e66416fc89d26f2d869e1c1b3a4291641f0fc939b0d025837c2697849e1dc81da49c53fc013e5336a9b848368e5347f134e51105b85563c4b5dd8a0b33bcf5e9e3831ff4d3c5848aa5e53c553b97cb5d5e848164fe8b053265722c7ed14bc7027c7e54fb42ef9189727a5561372ac7d24e2d15abad51d25833c4f852407b745ec0943191373d0db984d63353476283ac428cf9369e94fc9b637b0bbcc3fb294a9896749cd0434fa5d283973d63557c1c1a6244f2b294eb862835ab219a6b74a5d5e718fe1bfe04e0726f8fc19cdd0bff98f68381bc5935c112bc5eed29bed686f966f1fbcc9f09c21faa4e234754cd98e48ea163590b86e9466e24b339c71ebc2b3b52d3e4588b3d090c9ce4663a981922988af3c81d9450",
        "8ed361d19da7953073dcbeabe785561c5caf185b6d24d65e2d93af2f3d296baf094f76dfe1b15af84a49970ebc79655ca7bc8984906d13117634e8e489db75c6ec9604ad194c27ea05cec9cff2c7da64c6a77cc078483ac70cbe76d35cb3625a98fd833b12277dc01ddcb2765ab5caec11b8ee26cf5f4048e4aa9e7f0784b9e5420dcdacd8d14e335fd37453b2d8722c29d4ef329573c34e171edb30b2f2bb2b7af826bd2ec205d738b57dabce12001a4e0aa5ffd1d4674a8b2a2528ad6fd30ef941d031b563c53191033dc72a4a706a1922abbc79c5af06fa8ecb40ee92c56ebee92afc0064652775f9400882f2298e2916396967ffec34132a1aaf0fe1d025b2103458982cdaa447f7c05864e6cf4e63db9a4ac29022760155dea969454682344a9a9a7f483eef66f0651024db4c0a"],
    ];
    // No nonce
    let nonce = Nonce::default();

    for i in 0..TEST_KEYS.len() {
        let pk = bbs::PublicKey::from(&bbs::SecretKey::from_seed(TEST_KEYS[i]).unwrap());
        let gens = bbs::MessageGenerators::from_public_key(pk, test_atts.len());
        let sig = bbs::Signature::from_bytes(
            &<[u8; bbs::Signature::BYTES]>::try_from(hex::decode(EXPECTED_SIGS[i]).unwrap())
                .unwrap(),
        )
        .unwrap();
        // start with all hidden messages
        let mut proof_msgs: Vec<ProofMessage> = test_atts
            .iter()
            .map(|a| ProofMessage::Hidden(HiddenMessage::ProofSpecificBlinding(*a)))
            .collect();

        // Reveal 1 message at a time
        for j in 0..proof_msgs.len() {
            let res = bbs::PokSignature::init_with_rng(sig, &gens, proof_msgs.as_slice(), &mut rng);
            assert!(res.is_ok());
            let mut pok = res.unwrap();
            let mut hasher = sha3::Shake256::default();
            pok.add_proof_contribution(&mut hasher);
            hasher.update(nonce.to_bytes());
            let mut reader = hasher.finalize_xof();
            let mut data = [0u8; 48];
            reader.read(&mut data[..]);
            let c = Challenge::from_okm(&data);
            let res = pok.generate_proof(c);
            assert!(res.is_ok());

            let proof = res.unwrap();
            let e_proof = hex::decode(expected_proofs[i][j]).unwrap();
            proof_msgs[j] = ProofMessage::Revealed(test_atts[j]);
            assert_eq!(proof.to_bytes(), e_proof);

            let mut revealed_msgs = Vec::new();
            for k in 0..j {
                revealed_msgs.push((k, test_atts[k]));
            }

            assert!(bbs::Verifier::verify_signature_pok(
                revealed_msgs.as_slice(),
                pk,
                proof,
                &gens,
                nonce,
                c
            ));
        }
    }
}