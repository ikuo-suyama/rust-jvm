public class Types {
    public static void main(String[] args) {
        int i1 = 1001001001;
        int i2 = 2002002002;
        int i3 = i1 + i2;

        long l1 = 10101010101010L;
        long l2 = 20202020202020L;
        long l3 = l1 + l2;

        boolean b1 = true;
        boolean b2 = false;
        boolean b3 = b1 | b2;

        short s1 = 1;
        short s2 = 2;
        short s3 = (short)(s1 + s2);

        char c1 = 'a';
        char c2 = 'b';
        char c3 = (char)(c1 + c2);

        float f1 = 1.01f;
        float f2 = 2.01f;
        float f3 = f1 + f2;

        double d1 = 1.0e100;
        double d2 = 2.0e100;
        double d3 = d1 + d2;
    }
}
