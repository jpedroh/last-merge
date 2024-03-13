package de.fosd.jdime;

import java.io.File;
import java.net.URISyntaxException;
import java.net.URL;
import java.util.Arrays;

import org.junit.BeforeClass;

import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;
import static org.junit.Assert.fail;

public class JDimeTest {
    protected static File file(File parent, String child) {
        File f = new File(parent, child);
        assertTrue(f + " does not exist.", f.exists());

        return f;
    }

    protected static File file(File parent, String name, String... names) {

        if (names != null) {
            String path = String.format("%s/%s", name, String.join("/", names));
            return file(parent, path);
        } else {
            return file(parent, name);
        }
    }

    protected static File file(String path) {
        URL res = JDimeTest.class.getResource(path);

        assertNotNull("The file " + path + " was not found.", res);

        try {
            return new File(res.toURI());
        } catch (URISyntaxException e) {
            fail(e.getMessage());
            return null;
        }
    }

    protected static File file(String name, String... names) {

        if (names != null) {
            String path = String.format("/%s/%s", name, String.join("/", names));
            return file(path);
        } else {
            return file("/" + name);
        }
    }
}
