class ForLoop {
    public static void main(String[] args) {
        System.out.println(forLoop());
    }
    public static int forLoop() {
        int a = 0;
        for(int i = 0; i < 10000; i++) {
            a += i;
        }
        return a;
    }
}
