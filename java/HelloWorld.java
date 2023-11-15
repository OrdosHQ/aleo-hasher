public class HelloWorld {
    static {
        System.loadLibrary("aleo_hasher");
    }

    private native void rust_function();

    public static void main(String[] args) {
        new HelloWorld().rust_function();
    }
}