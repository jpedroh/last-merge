public class Main {
    static {
        System.out.println("I'm a static block");
    }

    static {
        int y = 2;
    }

    static {
        System.out.println("I don't know what's going on");
    }

    public Main() {
        System.out.println("I'm a constructor");
        int y = 3;
    }
}
