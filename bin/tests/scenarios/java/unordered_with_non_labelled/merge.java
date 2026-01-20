public class Main {
    static {
        int
<<<<<<<
x = 0
=======
y = 2
>>>>>>>
;
    }

    static {
        System.out.println("I'm a static block");
    }

    public Main() {
        System.out.println("I'm a constructor");
        int y = 3;
    }

    static {
        System.out.println("I don't know what's going on");
    }
}