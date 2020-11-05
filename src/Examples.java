public class Examples {
    void f() {
        String[] names = new String[] { "Nic", "Sue", "Tom" };
        g(names);
    }

    void g(String[] names) {
        // g can set all the names to whatever it wants!
        for (int i=0; i<names.length; ++i) {
            names[i] = null;
        }
    }
}