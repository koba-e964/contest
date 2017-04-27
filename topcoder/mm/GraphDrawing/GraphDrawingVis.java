import java.awt.*;
import java.awt.event.*;
import java.awt.image.*;
import java.io.*;
import java.util.*;
import java.security.*;
import javax.swing.*;

// --------------------------------------------------------
class Pnt {
    public int x,y;
    public Pnt() {};
    public Pnt(int x1, int y1) {
        x = x1;
        y = y1;
    }
    public boolean equals(Pnt other) {
        return (x == other.x && y == other.y);
    }
    public int dist2(Pnt other) {
        return (x-other.x)*(x-other.x) + (y-other.y)*(y-other.y);
    }
    public String toString() {
        return "(" + x + "," + y + ")";
    }
}

// --------------------------------------------------------
public class GraphDrawingVis {
    static int maxNV = 1000, minNV = 10;
    final int SZ = 700;             // field size
    
    int NV, NE;                     // number of vertices and number of edges in the graph
    int[] edgeBeg, edgeEnd;         // indices of edges start and ends
    int[] edgeLen;                  // required length of the edges
    
    Pnt[] p;                        // coordinates of points (returned by user)
    
    String errmes;                  // error message from score calculation
    double[] ratios;
    Pnt[] pt;                       // coordinates of points used during generation and visualization
    double min_ratio, max_ratio;
    double score;
    // ---------------------------------------------------
    void generate(String seedStr) {
      try {
        SecureRandom rnd = SecureRandom.getInstance("SHA1PRNG");
        long seed = Long.parseLong(seedStr);
        rnd.setSeed(seed);
        // number of points
        NV = rnd.nextInt(maxNV - minNV + 1) + minNV;
        if (seed == 1)
            NV = minNV / 2;
        else if (seed == 2)
            NV = minNV * 2;
        else if (seed == 3)
            NV = maxNV;

        int maxNE = NV * Math.min(minNV, (NV - 1) / 4), minNE = NV - 1;
        NE = rnd.nextInt(maxNE - minNE + 1) + minNE;
        
        // generate the required edges
        // each vertex must be used in at least one edge
        // no edge is given twice
        // graph doesn't have to be connected
        edgeBeg = new int[NE];
        edgeEnd = new int[NE];
        while (true) {
            boolean[] vertexUsed = new boolean[NV];
            int nvUsed = 0;
            HashSet<Integer> edgeUsed = new HashSet<>();
            for (int i = 0; i < NE; ++i) {
                while (true) {
                    // if there are unused vertices, pick one of them for beg
                    int beg;
                    if (nvUsed < NV) {
                        for (beg = 0; beg < NV; ++beg)
                            if (!vertexUsed[beg])
                                break;
                    } else 
                        beg = rnd.nextInt(NV);
                    int end = rnd.nextInt(NV);
                    if (end == beg)
                        continue;
                    if (beg > end) {
                        int t = beg;
                        beg = end;
                        end = t;
                    }
                    Integer edge = new Integer(beg * NV + end);
                    if (edgeUsed.contains(edge))
                        continue;
                    // valid new edge: add
                    edgeUsed.add(edge);
                    if (!vertexUsed[beg]) {
                        nvUsed++;
                        vertexUsed[beg] = true;
                    }
                    if (!vertexUsed[end]) {
                        nvUsed++;
                        vertexUsed[end] = true;
                    }
                    edgeBeg[i] = beg;
                    edgeEnd[i] = end;
                    break;
                }
            }
            if (nvUsed == NV)
                break;
        }
        
        // assign lengths (no special constraints)
        int maxLen = (int)((SZ+1) * Math.sqrt(2.0));
        int minLen = 1;
        
        // assign some point coordinates
        pt = new Pnt[NV];
        HashSet<Integer> pointUsed = new HashSet<>();
        for (int i = 0; i < NV; ++i) {
            while (true) {
                pt[i] = new Pnt(rnd.nextInt(SZ + 1), rnd.nextInt(SZ + 1));
                Integer point = new Integer(pt[i].x * (SZ + 1) + pt[i].y);
                if (pointUsed.contains(point))
                    continue;
                pointUsed.add(point);
                break;
            }
        }
        
        // calculate lengths based on the real values and distort a certain percentage of them
        int distort = rnd.nextInt(100);
        System.out.println("Number of vertices NV = " + NV);
        System.out.println("Number of edges NE = " + NE);
        System.out.println("Distortion percentage = " + distort);
        if (debug)
            System.out.println("Required edge lengths: ");

        edgeLen = new int[NE];
        for (int i = 0; i < NE; ++i) {
            double realLen = Math.sqrt(pt[edgeBeg[i]].dist2(pt[edgeEnd[i]]));
            if (rnd.nextDouble() * 100 < distort) {
                // distorted value
                edgeLen[i] = (int)(realLen * Math.pow(1.1, rnd.nextGaussian()));
            } else {
                edgeLen[i] = (int)realLen;
            }
            if (edgeLen[i] < minLen)
                edgeLen[i] = minLen;
            if (edgeLen[i] > maxLen)
                edgeLen[i] = maxLen;
            
            if (debug)
                System.out.println(edgeBeg[i] + "-" + edgeEnd[i] + " = " + edgeLen[i] + " (dist " + String.format("%.6f", edgeLen[i] / realLen) + ")");
        }

      }
      catch (Exception e) { 
        addFatalError("An exception occurred while generating test case.");
        e.printStackTrace(); 
      }
    }
    // ---------------------------------------------------
    double calcScore(boolean deb) {
        // calculate the score of current arrangement of points
        errmes = "";

        // verify that all points have distinct coordinates
        for (int i = 0; i < NV; ++i)
        for (int j = 0; j < i; ++j)
            if (p[i].equals(p[j])) {
                errmes = "Multiple vertices placed at " + p[i] + ".";
                return 0.0;
            }

        // for each edge, calculate the ratio "actual length / required length"
        min_ratio = -1;
        max_ratio = -1;
        for (int i = 0; i < NE; ++i) {
            double len = Math.sqrt(p[edgeBeg[i]].dist2(p[edgeEnd[i]]));
            double r =  len / edgeLen[i];
            if (min_ratio < 0 || min_ratio > r)
                min_ratio = r;
            if (max_ratio < r)
                max_ratio = r;
            ratios[i] = r;
            if (deb)
                System.out.println(edgeLen[i] + " " + len + " " + r);
        }
        if (deb) {
            System.out.println("Min ratio = " + min_ratio);
            System.out.println("Max ratio = " + max_ratio);
        }

        return min_ratio / max_ratio;
    }
    // ---------------------------------------------------
    public double runTest(String seed) {
      try {
        generate(seed);

        p = new Pnt[NV];
        ratios = new double[NE];
        if (proc != null) {
            // get the return and convert it into the coordinates of the vertices
            int[] edgePar = new int[3 * NE];
            for (int i = 0; i < NE; ++i) {
                edgePar[3*i] = edgeBeg[i];
                edgePar[3*i + 1] = edgeEnd[i];
                edgePar[3*i + 2] = edgeLen[i];
            }
            int[] ret;
            try { ret = plot(NV, edgePar); }
            catch (Exception e) {
                addFatalError("Failed to get result from plot.");
                return 0;
            }
            
            // coordinates of vertex i are given by ret[2*i], ret[2*i+1]
            if (ret.length != 2 * NV) {
                addFatalError("Your return must contain " + 2 * NV + " elements, and it contained " + ret.length + ".");
                return 0;
            }
            
            for (int i = 0; i < NV; ++i) {
                p[i] = new Pnt(ret[2*i], ret[2*i + 1]);
                if (p[i].x < 0 || p[i].x > SZ || p[i].y < 0 || p[i].y > SZ) {
                    addFatalError("Vertex " + i + " placed at " + p[i] + " outside of [0, " + SZ + "] x [0, " + SZ + "] field.");
                    return 0;
                }
            }
        }
        else {
            // use the points allocation from which the input data was generated
            p = pt;
        }

        if (vis) {
            // draw the image
            jf.setSize(SZX+17,SZY+37);
            jf.setVisible(true);
            draw();
        }

        if (debug) {
            System.out.println("Final vertices coordinates: ");
            for (int i = 0; i < NV; ++i)
                System.out.println(i + " - " + p[i]);
        }
        
        this.score = calcScore(debug);
        if (!errmes.equals(""))
            addFatalError(errmes);
        return this.score;
      }
      catch (Exception e) { 
        addFatalError("An exception occurred while trying to process your program's results.");
        e.printStackTrace(); 
        return 0.0;
      }
    }
// ------------- visualization part ----------------------
    static String exec;
    static boolean vis, debug, labels;
    static Process proc;
    JFrame jf;
    Vis v;
    InputStream is;
    OutputStream os;
    BufferedReader br;
    // problem-specific drawing params
    final int SZX = SZ+2+100,SZY=SZ+2;
    // ---------------------------------------------------
    int[] plot(int NV, int[] edges) throws IOException {
        // pass the params to the solution and get the return
        int i;
        StringBuffer sb = new StringBuffer();
        sb.append(NV).append('\n');
        sb.append(edges.length).append('\n');
        for (i = 0; i < edges.length; ++i)
            sb.append(edges[i]).append('\n');
        os.write(sb.toString().getBytes());
        os.flush();
        
        // get the return - an array of integers
        int nret = Integer.parseInt(br.readLine());
        int[] ret = new int[nret];
        for (i = 0; i < nret; ++i)
            ret[i] = Integer.parseInt(br.readLine());
        return ret;
    }
    // ---------------------------------------------------
    void draw() {
        if (!vis) return;
        v.repaint();
    }
    // ---------------------------------------------------
    public class Vis extends JPanel implements WindowListener {
        public void paint(Graphics g) {
          try {
            // do painting here
            char[] ch;
            BufferedImage bi = new BufferedImage(SZX+10,SZY+10,BufferedImage.TYPE_INT_RGB);
            Graphics2D g2 = (Graphics2D)bi.getGraphics();
            // background
            g2.setColor(new Color(0xD3D3D3));
            g2.fillRect(0,0,SZX+10,SZY+10);
            g2.setColor(Color.WHITE);
            g2.fillRect(0,0,SZ+1,SZ+1);
            // frame
            g2.setColor(Color.BLACK);
            g2.drawRect(0,0,SZ+1,SZ+1);

            // get current score and ratios
            double score = calcScore(false);

            // edges that have given lengths (with ratios)
            for (int i = 0; i < NE; ++i) {
                // assign color based on ratio: longer than perfect are red, shorter are violet
                int c = 0;
                if (ratios[i] >= 1.0) {
                    if (max_ratio < 1.0 + 1E-5)
                        c = 0;
                    else
                        c = 0x10000 * (int)((ratios[i] - 1.0) / (max_ratio - 1.0) * 0xF0+0xF);
                } else {
                    if (min_ratio > 1.0 - 1E-5)
                        c = 0;
                    else
                        c = 0x10001 * (int)((1.0 - ratios[i]) / (1.0 - min_ratio) * 0xF0+0xF);
                }
                g2.setColor(new Color(c));
                g2.drawLine(p[edgeBeg[i]].x, (SZ-1-p[edgeBeg[i]].y), p[edgeEnd[i]].x, (SZ-1-p[edgeEnd[i]].y));
                if (labels) {
                    g2.setColor(Color.BLUE);
                    ch = String.format("%.3f", ratios[i]).toCharArray();
                    g2.drawChars(ch,0,ch.length, (p[edgeBeg[i]].x+p[edgeEnd[i]].x)/2+2, SZ-1-(p[edgeBeg[i]].y+p[edgeEnd[i]].y)/2-2);
                }
            }

            // vertices (with numbers)
            g2.setColor(Color.BLACK);
            for (int i = 0; i < NV; i++) {
                g2.fillOval(p[i].x-2, SZ-1-p[i].y-2,5,5);
                if (labels) {
                    ch = (i+"").toCharArray();
                    g2.drawChars(ch,0,ch.length, p[i].x+2, SZ-1-p[i].y-2);
                }
            }

            ch = String.format("%.6f", score).toCharArray();
            g2.setFont(new Font("Arial",Font.BOLD,14));
            g2.drawChars(ch,0,ch.length,SZ+10,200);

            g.drawImage(bi,0,0,SZX+10,SZY+10,null);
      }
      catch (Exception e) { e.printStackTrace(); }
        }
        public Vis() {
            jf.addWindowListener(this);
        }
        // WindowListener
        public void windowClosing(WindowEvent e) { 
            if(proc != null)
                try { proc.destroy(); } 
                catch (Exception ex) { ex.printStackTrace(); }
            System.exit(0); 
        }
        public void windowActivated(WindowEvent e) { }
        public void windowDeactivated(WindowEvent e) { }
        public void windowOpened(WindowEvent e) { }
        public void windowClosed(WindowEvent e) { }
        public void windowIconified(WindowEvent e) { }
        public void windowDeiconified(WindowEvent e) { }
    }
    // ---------------------------------------------------
    public GraphDrawingVis(String seed) {
        //interface for runTest
        if (vis)
        {   jf = new JFrame();
            v = new Vis();
            jf.getContentPane().add(v);
        }
        if (exec != null) {
            try {
                Runtime rt = Runtime.getRuntime();
                proc = rt.exec(exec);
                os = proc.getOutputStream();
                is = proc.getInputStream();
                br = new BufferedReader(new InputStreamReader(is));
                new ErrorReader(proc.getErrorStream()).start();
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
        System.out.println("Score = "+runTest(seed));
        if (proc != null)
            try { proc.destroy(); } 
            catch (Exception e) { e.printStackTrace(); }
    }
    // ---------------------------------------------------
    public static void main(String[] args) {
	// Test all seeds in range [10000 .. 10000 + ntest)
	final int ntest = 50;
        vis = false;
        labels = false;
        for (int i = 0; i<args.length; i++)
	    {
		if (args[i].equals("-exec"))
		    exec = args[++i];
		if (args[i].equals("-vis"))
		    vis = true;
		if (args[i].equals("-debug"))
		    debug = true;
		if (args[i].equals("-labels"))
		    labels = true;
	    }
	if (vis || debug || labels) {
	    GraphDrawingVis f = new GraphDrawingVis("1");
	} else {
	    double sum = 0, sum_sq = 0;
	    for (int i = 0; i < ntest; ++i) {
		String seed = Integer.toString(10000 + i);
		System.out.println("\nCase #" + i + " (seed = " + seed + ")");
		GraphDrawingVis f = new GraphDrawingVis(seed);
		sum += f.score * 1e6;
		sum_sq += Math.pow(f.score * 1e6, 2);
	    }
	    double avrg = sum / ntest;
	    System.out.println("\nAverage = " + avrg);
	    if (ntest >= 2) {
		double stdev =  Math.sqrt((sum_sq - ntest * Math.pow(avrg, 2))
					  / (ntest - 1));
		System.out.println(" Uncertainty = \\pm "
				   + (stdev / Math.sqrt(ntest)));
		System.out.println("Stdev(expected) = " + stdev);
	    }
	}
    }
    // ---------------------------------------------------
    void addFatalError(String message) {
        System.out.println(message);
    }
}

class ErrorReader extends Thread{
    InputStream error;
    public ErrorReader(InputStream is) {
        error = is;
    }
    public void run() {
        try {
            byte[] ch = new byte[50000];
            int read;
            while ((read = error.read(ch)) > 0)
            {   String s = new String(ch,0,read);
                System.out.print(s);
                System.out.flush();
            }
        } catch(Exception e) { }
    }
}
