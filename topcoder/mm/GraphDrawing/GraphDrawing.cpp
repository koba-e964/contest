// C++11
#include <algorithm>
#include <cstdlib>
#include <cstdio>
#include <cmath>
#include <iostream>
#include <queue>
#include <vector>
#include <set>
#include <cassert>

using namespace std;
const double pi = 3.1415926535897932384626;

#define CONSIDERING_TIME 8.5

/*
 * Union-Find tree
 * header requirement: vector
 */
class UnionFind {
private:
  std::vector<int> disj;
  std::vector<int> rank;
public:
  UnionFind(int n) : disj(n), rank(n) {
    for (int i = 0; i < n; ++i) {
      disj[i] = i;
      rank[i] = 0;
    }
  }
  int root(int x) {
    if (disj[x] == x) {
      return x;
    }
    return disj[x] = root(disj[x]);
  }
  void unite(int x, int y) {
    x = root(x);
    y = root(y);
    if (x == y) {
      return;
    }
    if (rank[x] < rank[y]) {
      disj[x] = y;
    } else {
      disj[y] = x;
      if (rank[x] == rank[y]) {
	++rank[x];
      }
    }
  }
  bool is_same_set(int x, int y) {
    return root(x) == root(y);
  }
};

/*
 * https://topcoder.g.hatena.ne.jp/CKomaki/20141202
 */
class XorShift{
public:
  static int rand();
private:
  static unsigned int x;
  static unsigned int y;
  static unsigned int z;
  static unsigned int w;
  static unsigned int t;
};
unsigned int XorShift::x = 123456789;
unsigned int XorShift::y = 362436069;
unsigned int XorShift::z = 521288629;
unsigned int XorShift::w = 88675123;
unsigned int XorShift::t = 1;

int XorShift::rand()
{
  t = x ^ (x << 11);
  x = y;
  y = z;
  z = w;
  w = (w ^ (w >> 19)) ^ (t ^ (t >> 8));
  return w & 0x7fffffff;
}

#define rep(i, n) for(int i=0;i<int(n);++i)
typedef long long ll;

class GraphDrawing {
  // http://topcoder.g.hatena.ne.jp/CKomaki/20141202/1418158488
public:
  typedef pair<int, int> PI;
  static double cycle_per_sec;
  static double getTime(unsigned long long int begin_cycle)
  {
    return (double)(getCycle() - begin_cycle) / cycle_per_sec;
  }

  static unsigned long long int getCycle()
  {
    unsigned int low, high;
    __asm__ volatile ("rdtsc" : "=a" (low), "=d" (high));
    return ((unsigned long long int)low) | ((unsigned long long int)high << 32);
  }
  vector<vector<PI> > link;
  int n;
  struct State {
    const GraphDrawing &gd;
    vector<PI> pts;
    State(const GraphDrawing &gd, const vector<PI> &pts): gd(gd) {
      this->pts = pts;
    }
    State(const State &s): gd(s.gd), pts(s.pts) {}
    void operator=(const State &s) {
      this->pts = s.pts;
    }
    double score() const {
      double mi = 1e16;
      double ma = 1e-16;
      double delta = 0.0;
      int ec = 0;
      rep(i, gd.n) {
	rep(j, gd.link[i].size()) {
	  PI t(gd.link[i][j]);
	  if (i > t.first) { continue; }
	  ec += 1;
	  double dist = pow(pts[i].first - pts[t.first].first, 2)
	    + pow(pts[i].second - pts[t.first].second, 2);
	  double discr = dist / pow(t.second, 2);
	  mi = min(mi, discr);
	  ma = max(ma, discr);
	  delta += discr > 1 ? discr - 1 : 1 / discr - 1;
	}
      }
      return mi / ma * max(1 - 1e-4 * delta / ec, 1e-4);
    }
    double discrep_total() const {
      double delta = 0.0;
      rep(i, gd.n) {
	rep(j, gd.link[i].size()) {
	  PI t(gd.link[i][j]);
	  if (i > t.first) { continue; }
	  double dist = pow(pts[i].first - pts[t.first].first, 2)
	    + pow(pts[i].second - pts[t.first].second, 2);
	  double discr = dist / pow(t.second, 2);
	  delta += discr > 1 ? discr - 1 : 1 / discr - 1;
	}
      }
      return delta;
    }
    void debug() const {
      rep(i, gd.n) {
	rep(j, gd.link[i].size()) {
	  PI t(gd.link[i][j]);
	  double dist = pow(pts[i].first - pts[t.first].first, 2)
	    + pow(pts[i].second - pts[t.first].second, 2);
	  dist = sqrt(dist);
	  double discr = dist/t.second;
	  if (discr < 1.0) discr = 1 / discr;
	  if (discr >= 5) {
	    fprintf(stderr, "%d-%d (%d): %f (discr = %f)\n", i, t.first, t.second, dist, dist / t.second);
	  }
	}
      }
#if 0
      vector<pair<PI, int> > pool;
      rep(i, gd.n) {
	pool.push_back(make_pair(pts[i], i));
      }
      sort(pool.begin(), pool.end());
      rep(i, gd.n) {
	pair<PI, int> t = pool[i];
	fprintf(stderr, "vertex %d (%d, %d)\n", t.second, t.first.first, t.first.second);
      }
#endif
    }
  };
  void improve(State &s, int v, bool &modified, PI &orig) const {
    double maxratio = 0.0;
    PI maxi = PI(-1, -1);
    set<PI> others;
    rep(i, link.size()) {
      if (i != v) {
	others.insert(s.pts[i]);
      }
    }
    int orig_x = s.pts[v].first;
    int orig_y = s.pts[v].second;
    rep(ti, 40) {
      int i = orig_x + (ti % 2 ? (ti / 2 + 1) * (ti / 2 + 1) : -(ti / 2) * (ti / 2));
      if (i < 0 || i > 700) { continue; }
      rep(tj, 40) {
	int j = orig_y + (tj % 2 ? (tj / 2 + 1) * (tj / 2 + 1) : -(tj / 2) * (tj / 2));
	if (j < 0 || j > 700) { continue; }
	// rep(j, 701) {
	if (others.count(PI(i, j))) { continue; }
	double mi = 1e16;
	double ma = 1e-16;
	double delta = 0.0;
	rep(k, link[v].size()) {
	  int w = link[v][k].first;
	  double des = link[v][k].second * link[v][k].second;
	  double dist = (i - s.pts[w].first) * (i - s.pts[w].first)
	    + (j - s.pts[w].second) * (j - s.pts[w].second);
	  mi = min(mi, dist / des);
	  ma = max(ma, dist / des);
	  delta += dist > des ? dist / des - 1 : des / dist - 1;
	  if (mi / ma <= maxratio) { break; }
	}
	double ratio = mi / ma * max(1.0 - 1e-4 * delta / link[v].size(), 1e-4);
	if (maxratio < ratio) {
	  maxratio = ratio;
	  maxi = PI(i, j);
	}
      }
    }
    // move
    assert (maxi.first >= 0);
    modified = s.pts[v] != maxi;
    orig = s.pts[v];
    s.pts[v] = maxi;
  }
  void improve_edge(State &s, int v, int w, bool &modified, vector<pair<int, PI> > &orig_pts) const {
    double maxratio = 0.0;
    PI maxi = PI(-1, -1), maxi2(-1, -1);
    set<PI> others;
    rep(i, link.size()) {
      if (i != v) {
	others.insert(s.pts[i]);
      }
    }
    int orig_x = s.pts[v].first;
    int orig_y = s.pts[v].second;
    int orig_x2 = s.pts[w].first;
    int orig_y2 = s.pts[w].second;
    rep(ti, 40) {
      int i = orig_x + (ti % 2 ? (ti / 2 + 1) * (ti / 2 + 1) : -(ti / 2) * (ti / 2));
      if (i < 0 || i > 700) { continue; }
      rep(tj, 40) {
	int j = orig_y + (tj % 2 ? (tj / 2 + 1) * (tj / 2 + 1) : -(tj / 2) * (tj / 2));
	if (j < 0 || j > 700) { continue; }
	rep(ti2, 40) {
	  int i2 = orig_x2 + (ti2 % 2 ? (ti2 / 2 + 1) * (ti2 / 2 + 1) : -(ti2 / 2) * (ti2 / 2));
	  if (i2 < 0 || i2 > 700) { continue; }
	  rep(tj2, 40) {
	    int j2 = orig_y2 + (tj2 % 2 ? (tj2 / 2 + 1) * (tj2 / 2 + 1) : -(tj2 / 2) * (tj2 / 2));
	    if (j2 < 0 || j2 > 700) { continue; }
	    if (PI(i, j) == PI(i2, j2)) { continue; }
	    if (others.count(PI(i, j))) { continue; }
	    if (others.count(PI(i2, j2))) { continue; }
	    double mi = 1e16;
	    double ma = 1e-16;
	    double delta = 0.0;
	    rep(k, link[v].size()) {
	      int u = link[v][k].first;
	      double des = link[v][k].second * link[v][k].second;
	      PI targ = u == w ? PI(i2, j2) : s.pts[u]; 
	      double dist = (i - targ.first) * (i - targ.first)
		+ (j - targ.second) * (j - targ.second);
	      mi = min(mi, dist / des);
	      ma = max(ma, dist / des);
	      delta += dist > des ? dist / des - 1 : des / dist - 1;
	      if (mi / ma <= maxratio) { break; }
	    }
	    rep(k, link[w].size()) {
	      int u = link[w][k].first;
	      double des = link[w][k].second * link[w][k].second;
	      PI targ = u == v ? PI(i, j) : s.pts[u]; 
	      double dist = (i2 - targ.first) * (i2 - targ.first)
		+ (j2 - targ.second) * (j2 - targ.second);
	      mi = min(mi, dist / des);
	      ma = max(ma, dist / des);
	      delta += dist > des ? dist / des - 1 : des / dist - 1;
	      if (mi / ma <= maxratio) { break; }
	    }
	    double ratio = mi / ma * max(1.0 - 1e-4 * delta / link[v].size(), 1e-4);
	    if (maxratio < ratio) {
	      maxratio = ratio;
	      maxi = PI(i, j);
	      maxi2 = PI(i2, j2);
	    }
	  }
	}
      }
    }
    // move
    assert (maxi.first >= 0);
    modified = s.pts[v] != maxi || s.pts[w] != maxi2;
    orig_pts.push_back(make_pair(v, s.pts[v]));
    orig_pts.push_back(make_pair(w, s.pts[w]));
    s.pts[v] = maxi;
    s.pts[w] = maxi2;
  }
  void climb_hill(State &s, int cnt, vector<pair<int, PI> > &orig_pts) {
    rep(i, cnt) {
      pair<double, PI> max_discrep(0.0, PI(-1, -1));
      rep(v, n) {
	rep(k, link[v].size()) {
	  int w = link[v][k].first;
	  double des = link[v][k].second;
	  double d = pow(s.pts[v].first - s.pts[w].first, 2)
	    + pow(s.pts[v].second - s.pts[w].second, 2);
	  d /= des * des;
	  if (d < 1.0) { d = 1.0 / d; }
	  max_discrep = max(max_discrep, make_pair(d, XorShift::rand() % 2 ? PI(v, w) : PI(w, v)));
	}
      }
      bool modified;
      int v = max_discrep.second.first;
      PI orig_pt;
      improve(s, v, modified, orig_pt);
      if (not modified) { break; }
      orig_pts.push_back(pair<int, PI>(v, orig_pt));
    }
  }
  void climb_hill_edge(State &s, int cnt, vector<pair<int, PI> > &orig_pts) {
    rep(i, cnt) {
      pair<double, PI> max_discrep(0.0, PI(-1, -1));
      rep(v, n) {
	rep(k, link[v].size()) {
	  int w = link[v][k].first;
	  double des = link[v][k].second;
	  double d = pow(s.pts[v].first - s.pts[w].first, 2)
	    + pow(s.pts[v].second - s.pts[w].second, 2);
	  d /= des * des;
	  if (d < 1.0) { d = 1.0 / d; }
	  max_discrep = max(max_discrep, make_pair(d, PI(v, w)));
	}
      }
      bool modified;
      int v = max_discrep.second.first;
      int w = max_discrep.second.second;
      PI orig_pt;
      improve_edge(s, v, w, modified, orig_pts);
      if (not modified) { break; }
    }
  }
  void dfs_sub(const vector<vector<PI> > &mst, State &s,
	       int v, int par, set<PI> &used, PI coord) const {
    int x = coord.first;
    int y = coord.second;
    s.pts[v] = PI(x, y);
    used.insert(PI(x, y));
    int dir_int = XorShift::rand() % 1000;
    rep(j, mst[v].size()) {
      PI w = mst[v][j];
      if (par == w.first) { continue; }
      double dir = (dir_int / 1000.0 + (double)j / mst[v].size()) * 2 * pi;
      int x_goal = x + w.second * cos(dir);
      int y_goal = y + w.second * sin(dir);
      double mindist = 1e14;
      PI mini(-1, -1);
      bool found = false;
      for (int nx = max(int(x - w.second * 1.5), 0); nx <= min(int(x + w.second * 1.5), 700);
	   ++nx) {
	if ((nx - x_goal) * (nx - x_goal) >= mindist) { continue; }
	for (int ny = max(int(y - w.second * 1.5), 0); ny <= min(int(y + w.second * 1.5), 700);
	     ++ny) {
	  if (used.count(PI(nx, ny))) { continue; }
	  double d = pow(nx - x_goal, 2) + pow(ny - y_goal, 2);
	  if (mindist > d) {
	    mindist = d;
	    found = true;
	    mini = PI(nx, ny);
	  }
	}
      }
      if (found) {
	dfs_sub(mst, s, w.first, v, used, mini);
      } else {
	fprintf(stderr, "randomised: vertex %d\n", w.first);
	int x = -1, y = -1;
	do {
	  x = XorShift::rand() % 701;
	  y = XorShift::rand() % 701;
	} while (used.count(PI(x, y)));
	dfs_sub(mst, s, w.first, v, used, PI(x, y));
      }
    }
  }
  void init_with_mst(const vector<int> &edges, State &s) const {
    // MinSpanTree
    vector<vector<PI> > mst(this->n);
    {
      UnionFind uf(this->n);
      vector<pair<int, PI> > es;
      rep(i, edges.size() / 3) {
	int u = edges[3 * i];
	int v = edges[3 * i + 1];
	int desired = edges[3 * i + 2];
	es.push_back(make_pair(desired, PI(u, v)));
      }
      sort(es.begin(), es.end());
      rep(j, es.size()) {
	pair<int, PI> cur = es[j];
	int u = cur.second.first;
	int v = cur.second.second;
	if (not uf.is_same_set(u, v)) {
	  uf.unite(u, v);
	  mst[u].push_back(PI(v, cur.first));
	  mst[v].push_back(PI(u, cur.first));
	}
      }
    }
    set<PI> used;
    vector<pair<int, PI> > que;
    int x = 350;
    int y = 350;

    dfs_sub(mst, s, 0, -1, used, PI(x, y));
  }
  void init_with_rand(const vector<int> &edges, State &s) const {
    (void) edges; // Suppress warning (unused variable)
    set<PI> used;
    for (int i = 0; i < this->n; ++i) {
      int x = -1;
      int y = -1;
      do {
	x = XorShift::rand() % 701;
	y = XorShift::rand() % 701;
      } while (used.count(PI(x, y)));
      s.pts[i] = PI(x, y);
      used.insert(PI(x, y));
    }
  }
  pair<double, vector<int> > plot_hill(int N, vector<int> edges) {
    this->n = N;
    ll starting_cycle = getCycle();
    this->link = vector<vector<PI> >(N);
    for (int i = 0; i < (int) edges.size() / 3; ++i) {
      int u = edges[3 * i];
      int v = edges[3 * i + 1];
      int desired = edges[3 * i + 2];
      link[u].push_back(PI(v, desired));
      link[v].push_back(PI(u, desired));
    }
    State s(*this, vector<PI>(N));
    // Pick a good initial state
    init_with_rand(edges, s);
    {
      vector<pair<int, PI> > orig_pts;
      climb_hill(s, 5, orig_pts);
    }
    if (s.score() < 1e-5) {
      init_with_mst(edges, s);
      fprintf(stderr, "MST ");
    }
    fprintf(stderr, "init %.4fsec\n", getTime(starting_cycle));
    int tick_cnt = 0;
    double s_sc = s.score();
    double maxscore = s_sc;
    State ret_s(s);
    vector<pair<int, PI> > orig_pts;
    int wallow_cnt = 0;
    for (int i = 0; getTime(starting_cycle) <= CONSIDERING_TIME; ++i) {
      double time_prop = getTime(starting_cycle) / CONSIDERING_TIME;
      bool debug = 
#ifdef DEBUG
        getTime(starting_cycle) >= tick_cnt
#else
      false
#endif
	;
      if (debug) {
	tick_cnt += 1;
	assert (s_sc == s.score());
      }
      if (debug) {
	fprintf(stderr, "step %d: ", i);
      }
      double cur_sc = s_sc;
      orig_pts.clear();
      climb_hill(s, 5, orig_pts);
      cur_sc = s.score();
      if (s_sc < cur_sc) {
	wallow_cnt = 0;
      } else {
	// restore changes
	for (int i = orig_pts.size() - 1; i >= 0; --i) {
	  s.pts[orig_pts[i].first] = orig_pts[i].second;
	}
	cur_sc = s_sc;
	orig_pts.clear();
	int v = XorShift::rand() % N;
	set<PI> others;
	rep(i, link.size()) {
	  if (i != v) {
	    others.insert(s.pts[i]);
	  }
	}
	int x = -1;
	int y = -1;
        do {
	  x = XorShift::rand() % 701;
	  y = XorShift::rand() % 701;
	} while (others.count(PI(x, y)));
	PI restore = s.pts[v];
	s.pts[v] = PI(x, y);
        climb_hill(s, 10, orig_pts);
	cur_sc = s.score();
	double ratio = cur_sc / max(s_sc, 1e-7);
	bool accept = ratio > 0.95;
	if (accept) {
	  if (debug) {
	    fprintf(stderr, "(perturb) ");
	  }
	  wallow_cnt = 0;
	} else {
	  // restore
	  for (int i = orig_pts.size() - 1; i >= 0; --i) {
	    s.pts[orig_pts[i].first] = orig_pts[i].second;
	  }
	  s.pts[v] = restore;
	  cur_sc = s_sc;
	  wallow_cnt += 1;
	  if (n > 150 && XorShift::rand() % 10000 < wallow_cnt) {
	    wallow_cnt = 0;
	    climb_hill_edge(s, 1, orig_pts);
	    cur_sc = s.score();
	    fprintf(stderr, "(edge)\n");
	  }
	}
      }
      double new_sc = cur_sc;
      if (debug) {
	fprintf(stderr, "score = %.6f (discrep_tot = %f)\n", new_sc, s.discrep_total());
      }
      if (maxscore < new_sc) {
	maxscore = new_sc;
	ret_s = s;
      }
      s_sc = new_sc;
    }
    vector<int> ret;
    for (int i = 0; i < N; ++i) {
      ret.push_back(ret_s.pts[i].first);
      ret.push_back(ret_s.pts[i].second);
    }
    // debug
    return make_pair(maxscore, ret);
  }
  vector<int> plot(int N, vector<int> edges) {
    //pair<double, vector<int> > res_beam = plot_beam(N, edges);
    pair<double, vector<int> > res_hill = plot_hill(N, edges);
    double sc_hill = res_hill.first;
    //double sc_beam = res_beam.first;
    /*
    if (sc_hill > sc_beam) {
      fprintf(stderr, "HILL %.6f > %.6f beam\n", sc_hill, sc_beam);
    } else {
      fprintf(stderr, "hill %.6f < %.6f BEAM\n", sc_hill, sc_beam);
    }
    return sc_hill > sc_beam ? res_hill.second : res_beam.second;
    */
    fprintf(stderr, "HILL %.6f\n", sc_hill);
    return res_hill.second;
  }
};
double GraphDrawing::cycle_per_sec = 2.5e9; // on TopCoder


// -------8<------- end of solution submitted to the website -------8<-------

template<class T> void getVector(vector<T>& v) {
  for (int i = 0; i < (int) v.size(); ++i)
    cin >> v[i];
}

int main() {
    GraphDrawing gd;
    GraphDrawing::cycle_per_sec = 1.9e9; // local
    int N;
    cin >> N;
    int E;
    cin >> E;
    vector<int> edges(E);
    getVector(edges);
    
    vector<int> ret = gd.plot(N, edges);
    cout << ret.size() << endl;
    for (int i = 0; i < (int)ret.size(); ++i)
        cout << ret[i] << endl;
    cout.flush();
}
