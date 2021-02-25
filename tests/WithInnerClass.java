class WithInnerClass {

    static class Data {
        private final int value;

        public Data(int value) {
            this.value = value;
        }

        public int getValue() {
            int a = value;

            if (a % 2 == 0) {
                return a;
            }
            return a + 1;
        }
    }
    
    public static void main(String[] args) {
        Data data = new Data(42);

        System.out.println("Value: " + data.getValue());
    }
}
