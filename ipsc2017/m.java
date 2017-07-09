import java.util.*;
import java.io.*;
import java.awt.image.*;
import javax.swing.*;
import javax.imageio.*;

class Point {
    int x, y;
    Point(int x, int y) {
	this.x = x;
	this.y = y;
    }
    @Override
    public boolean equals(Object o) {
	if (o instanceof Point) {
	    return equals((Point) o);
	}
	return false;
    }
    boolean equals(Point p) {
	return x == p.x && y == p.y;
    }
    @Override
    public String toString() {
	return x + "," + y;
    }
    @Override
    public int hashCode() {
	return x * 101157 + y;
    }
}

class Main {
    public static void main(String[] args) throws Exception {
	new Main();
    }
    static int width(List<Point> list) {
	int mi = (int) 1e8;
	int ma = (int) -1e8;
	for (Point p: list) {
	    mi = Math.min(mi, p.x);
	    ma = Math.max(ma, p.x);
	}
	return ma - mi;
    }
    static Point leftMost(List<Point> list) {
	int x = (int) 1e8;
	int y = (int) 1e8;
	for (Point p: list) {
	    x = Math.min(x, p.x);
	    y = Math.min(y, p.y);
	}
	return new Point(x, y);
    }
    static List<Point> relativize(List<Point> list, Point d) {
	List<Point> ret = new ArrayList<Point>();
	for (Point p: list) {
	    ret.add(new Point(p.x - d.x, p.y - d.y));
	}
	return ret;
    }
    static <T> double jaccard(Set<T> a, Set<T> b) {
	int sz_inter = 0;
	for (T u: a) {
	    if (b.contains(u)) {
		sz_inter += 1;
	    }
	}
	int sz_union = a.size() + b.size() - sz_inter;
	return (double) sz_inter / sz_union;
    }
    static double variance(Set<Point> a) {
        double x = 0, y = 0;
	double xsq = 0, ysq = 0;
	double n = a.size();
	for (Point p: a) {
	    x += p.x;
	    y += p.y;
	    xsq += p.x * p.x;
	    ysq += p.y * p.y;
	}
	return (xsq + ysq - x * x / n - y * y / n) / n;
    }
    static double area(Set<Point> a) {
	return a.size();
    }
    int w, h;
    int white;
    boolean[][] vis;
    BufferedImage readImage = null;
    List<List<Point>> pool;
    List<Point> current;
    HashSet<Point>[] glyph;
    void bfs(int xx, int yy) {
	int[] dx = {1, 0, -1, 0};
	int[] dy = {0, 1, 0, -1};
	Queue<Point> que = new ArrayDeque<Point>();
	que.add(new Point(xx, yy));
	while (!que.isEmpty()) {
	    Point p = que.remove();
	    int x = p.x;
	    int y = p.y;
	    if (vis[x][y]) { continue; }
	    vis[x][y] = true;
	    if (white == readImage.getRGB(x, y)) {
	        continue;
	    }
	    current.add(new Point(x, y));
	    for (int d = 0; d < 4; ++d) {
		int nx = x + dx[d];
		int ny = y + dy[d];
		if (nx < 0 || nx >= this.w || ny < 0 || ny >= this.h) {
		    continue;
		}
		que.add(new Point(nx, ny));
	    }
	}
    }
    Main() throws Exception {
	readImage = ImageIO.read(new File("m.png"));
        this.white = readImage.getRGB(0, 0);
        this.w = readImage.getWidth();
	this.h = readImage.getHeight();
	System.out.println("w = " + this.w + ", h = " + this.h);
	this.pool = new ArrayList<>();
	this.vis = new boolean[w][h];
	for (int x = 0; x < w; ++x) {
	    for (int y = 0; y < h; ++y) {
		if (vis[x][y] || white == readImage.getRGB(x, y)) {
		    continue;
		}
		this.current = new ArrayList<Point>();
	        bfs(x, y);
		this.pool.add(this.current);
	    }
	}
	System.out.println("conn = " + this.pool.size());
	this.glyph = new HashSet[100];
	for (List<Point> cc: this.pool) {
	    // Graph: 200 - 300
	    int width = width(cc);
	    Point leftUp = leftMost(cc);
	    if (150 <= width && width <= 300) {
		int magic = 354; // size of squares
		int idx_x = leftUp.x / magic;
		int idx_y = leftUp.y / magic;
		glyph[idx_x * 10 + idx_y] = new HashSet(relativize(cc, leftUp));
	    }
	}
	for (int i = 0; i < 100; ++i) {
		if (glyph[i] == null) {
		    System.err.println("null!!" + i);
	    }
	}
	double diff_mi = 1e8;
	for (int i = 0; i < 100; ++i) {
	    for (int j = i + 1; j < 100; ++j) {
		/*
		double jacc = jaccard(glyph[i], glyph[j]);
		jacc_ma = Math.max(jacc_ma, jacc);
		if (jacc >= 0.79) {
		    System.out.println("[identical] " + i + " === " + j);
		}
		*/
		double ai = area(glyph[i]);
		double aj = area(glyph[j]);
		double vi = variance(glyph[i]);
		double vj = variance(glyph[j]);
		double w = Math.pow((ai - aj) / ai, 2) + Math.pow((vi - vj) / vi, 2);
		diff_mi = Math.min(diff_mi, w);
		if (w <= 1e-5) {
		    System.out.println("[identical] " + i + " === " + j);
		}
	    }
	}
	System.err.println("diff_mi = " + diff_mi);
    }
}
