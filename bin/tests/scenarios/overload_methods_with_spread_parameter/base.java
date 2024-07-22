package de.fosd.jdime;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.StringReader;
import java.net.URL;
import java.util.Arrays;
import java.util.Iterator;
import java.util.*;

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

    protected static File file(String path) throws Exception {
        URL res = JDimeTest.class.getResource(path);

        assertNotNull("The file " + path + " was not found.", res);
        return new File(res.toURI());
    }

    protected static File file(String name, String... names) throws Exception {

        if (names != null) {
            String path = String.format("/%s/%s", name, String.join("/", names));
            return file(path);
        } else {
            return file("/" + name);
        }
    }
}
