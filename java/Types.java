public class Types {
    public static void main(String[] args) {
        int i = 1;
        int i2 = 2;
        int i3 = i + i2;

        long l = 10101010101010L;
        long l2 = 20202020202020L;
        long l3 = l + l2;

        boolean b = true;
        boolean b2 = false;
        boolean b3 = b | b2;

        short s = 1;
        short s2 = 2;
        short s3 = (short)(s + s2);

        char c = 'a';
        char c2 = 'b';
        char c3 = (char)(c + c2);

        float f = 1.01f;
        float f2 = 2.01f;
        float f3 = f + f2;

        double d = 1.0e100;
        double d2 = 2.0e100;
        double d3 = d + d2;
    }
}
