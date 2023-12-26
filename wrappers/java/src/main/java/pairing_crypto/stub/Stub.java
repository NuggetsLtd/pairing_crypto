package pairing_crypto;

public class Stub {
    static {
        System.loadLibrary("pairing_crypto_jni");
    }
     static native long add(long a, long b);

    public long addw(long a, long b) throws Exception {
        return add(a, b);
    }

}
