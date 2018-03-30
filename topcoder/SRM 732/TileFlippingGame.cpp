#include <cstdio>
#include <cmath>
#include <cstring>
#include <ctime>
#include <iostream>
#include <algorithm>
#include <set>
#include <vector>
#include <sstream>
#include <typeinfo>
#include <fstream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

const int DEBUG = 0;

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const ll inf = 1e15;

class TileFlippingGame {
  int calc(int n, int m, const vector<string> &X, char col, const vector<VL> &dis) {
    vector<bool> vis(n * m, false);
    int tot = 0;
    while (true) {
      pair<ll, PI> mi(inf, PI(-1, -1));
      REP(i, 0, n * m) {
	int x = i / m, y = i % m;
	if (X[x][y] == 'e') continue;
	pair<ll, PI> ma(-1, PI(-1, -1));
	if (vis[i]) continue;
	REP(j, 0, n * m) {
	  if (not vis[j] && dis[i][j] < inf) {
	    ma = max(ma, make_pair(dis[i][j], PI(i, j)));
	  }
	}
	if ((X[x][y] != col) ^ (ma.first % 2)) ma.first += 1;
	if (ma.second.first != -1) {
	  mi = min(mi, ma);
	}
      }
      if (mi.second.first == -1) break;
      if (DEBUG) {
	cerr << "hit " << mi.second.first << " " << mi.second.second << endl;
      }
      int d = mi.first;
      int nx = mi.second.first / m;
      int ny = mi.second.first % m;
      if (X[nx][ny] != 'e' &&d >= 0) {
	tot += d;
      }
      REP(j, 0, n * m) {
	if (dis[mi.second.first][j] < inf) {
	  vis[j] = true;
	}
      }
    }
    return tot;
  }
    public:
  int HowManyMoves(int n, int m, vector<string> X) {
    vector<VL> dis(n * m, VL(n * m, inf));
    int dx[4] = {1, 0, -1, 0};
    int dy[4] = {0, 1, 0, -1};
    REP(i, 0, n) {
      REP(j, 0, m) {
	REP(d, 0, 4) {
	  int nx = i + dx[d];
	  int ny = j + dy[d];
	  if (nx < 0 || nx >= n || ny < 0 || ny >= m) continue;
	  char u = X[i][j];
	  char v = X[nx][ny];
	  ll cos = u != 'e' && v != 'e' ? u != v : inf;
	  dis[i * m + j][nx * m + ny] = cos;
	}
      }
    }
    REP(i, 0, n * m) dis[i][i] = 0;
    REP(k, 0, n * m) {
      REP(i, 0, n * m) {
	REP(j, 0, n * m) {
	  dis[i][j] = min(dis[i][j], dis[i][k] + dis[k][j]);
	}
      }
    }
    int bl = calc(n, m, X, 'b', dis),
      wh = calc(n, m, X, 'w', dis);
    return min(wh, bl);
  }
};

// CUT begin
ifstream data("TileFlippingGame.sample");

string next_line() {
    string s;
    getline(data, s);
    return s;
}

template <typename T> void from_stream(T &t) {
    stringstream ss(next_line());
    ss >> t;
}

void from_stream(string &s) {
    s = next_line();
}

template <typename T> void from_stream(vector<T> &ts) {
    int len;
    from_stream(len);
    ts.clear();
    for (int i = 0; i < len; ++i) {
        T t;
        from_stream(t);
        ts.push_back(t);
    }
}

template <typename T>
string to_string(T t) {
    stringstream s;
    s << t;
    return s.str();
}

string to_string(string t) {
    return "\"" + t + "\"";
}

bool do_test(int n, int m, vector<string> X, int __expected) {
    time_t startClock = clock();
    TileFlippingGame *instance = new TileFlippingGame();
    int __result = instance->HowManyMoves(n, m, X);
    double elapsed = (double)(clock() - startClock) / CLOCKS_PER_SEC;
    delete instance;

    if (__result == __expected) {
        cout << "PASSED!" << " (" << elapsed << " seconds)" << endl;
        return true;
    }
    else {
        cout << "FAILED!" << " (" << elapsed << " seconds)" << endl;
        cout << "           Expected: " << to_string(__expected) << endl;
        cout << "           Received: " << to_string(__result) << endl;
        return false;
    }
}

int run_test(bool mainProcess, const set<int> &case_set, const string command) {
    int cases = 0, passed = 0;
    while (true) {
        if (next_line().find("--") != 0)
            break;
        int n;
        from_stream(n);
        int m;
        from_stream(m);
        vector<string> X;
        from_stream(X);
        next_line();
        int __answer;
        from_stream(__answer);

        cases++;
        if (case_set.size() > 0 && case_set.find(cases - 1) == case_set.end())
            continue;

        cout << "  Testcase #" << cases - 1 << " ... ";
        if ( do_test(n, m, X, __answer)) {
            passed++;
        }
    }
    if (mainProcess) {
        cout << endl << "Passed : " << passed << "/" << cases << " cases" << endl;
        int T = time(NULL) - 1522374789;
        double PT = T / 60.0, TT = 75.0;
        cout << "Time   : " << T / 60 << " minutes " << T % 60 << " secs" << endl;
        cout << "Score  : " << 250 * (0.3 + (0.7 * TT * TT) / (10.0 * PT * PT + TT * TT)) << " points" << endl;
    }
    return 0;
}

int main(int argc, char *argv[]) {
    cout.setf(ios::fixed, ios::floatfield);
    cout.precision(2);
    set<int> cases;
    bool mainProcess = true;
    for (int i = 1; i < argc; ++i) {
        if ( string(argv[i]) == "-") {
            mainProcess = false;
        } else {
            cases.insert(atoi(argv[i]));
        }
    }
    if (mainProcess) {
        cout << "TileFlippingGame (250 Points)" << endl << endl;
    }
    return run_test(mainProcess, cases, argv[0]);
}
// CUT end
