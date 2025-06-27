public class Main {
    static {
        int x = 0;
    }

    static {
        System.out.println("I'm a static block");
    }

    public Main() {
        System.out.println("I'm a constructor");
    }
}
