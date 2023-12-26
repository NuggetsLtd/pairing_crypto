package pairing_crypto;

import org.junit.Test;

public class StubTest {
   private void addHelper(pairing_crypto.Stub stub) {
       long a = 1;
       long b = 2;
       try {
           var x = stub.addw(a, b);
           var g= x;
       } catch (Exception e) {
           e.printStackTrace();
       }

   }

    @Test
    public void shouldAddNumbers() {
        addHelper(new Stub());
    }
}

