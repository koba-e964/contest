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

typedef long long ll;
const ll mod = 1e9 + 7;

using namespace std;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"


ll powmod(ll a, ll e) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}
void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int N = 100100;
const int M = 310;
bool initialised;
ll tbl[N][M];
ll b1[M][M];

void init(void) {
  if (initialised) return;
  initialised = true;
  tbl[0][0] = 1;
  REP(i, 1, N) {
    REP(j, 0, M) {
      add(tbl[i][j], tbl[i - 1][j] * i);
      if(j > 0)
	add(tbl[i][j], tbl[i - 1][j - 1]);
    }
  }
  REP(k, 0, M) {
    ll cur = k % 2 != 0 ? mod - 1 : 1;
    REP(i, 0, k + 1) {
      ll tmp = 1;
      REP(j, 0, M) {
	add(b1[k][j], cur * tmp);
	tmp = tmp * i % mod;
      }
      cur = cur * (mod + i - k) % mod;
      cur = cur * powmod(i + 1, mod - 2) % mod;
    }
  }
  if (0) {
    REP(i, 0, 5) {
      REP(j, 0, 5) {
	cerr << " " << tbl[i][j];
      }
      cerr << endl;
    }
  }
}

class CyclesNumber {
    public:
    vector<int> getExpectation(vector<int> n, vector<int> m) {
      int q = n.size();
      init();
      vector<int> ret;
      REP(i, 0, q) {
	int nn = n[i], mm = m[i];
	ll tot = 0;
	REP(i, 0, M) {
	  add(tot, tbl[nn][i] * b1[i][mm]);
	}
	ret.push_back(tot);
      }
      return ret;
    }
};

// CUT begin
ifstream data("CyclesNumber.sample");

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

template <typename T> string to_string(vector<T> ts) {
    stringstream s;
    s << "[ ";
    for (int i = 0; i < ts.size(); ++i) {
        if (i > 0) s << ", ";
        s << to_string(ts[i]);
    }
    s << " ]";
    return s.str();
}

bool do_test(vector<int> n, vector<int> m, vector<int> __expected) {
    time_t startClock = clock();
    CyclesNumber *instance = new CyclesNumber();
    vector<int> __result = instance->getExpectation(n, m);
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
        vector<int> n;
        from_stream(n);
        vector<int> m;
        from_stream(m);
        next_line();
        vector<int> __answer;
        from_stream(__answer);

        cases++;
        if (case_set.size() > 0 && case_set.find(cases - 1) == case_set.end())
            continue;

        cout << "  Testcase #" << cases - 1 << " ... ";
        if ( do_test(n, m, __answer)) {
            passed++;
        }
    }
    if (mainProcess) {
        cout << endl << "Passed : " << passed << "/" << cases << " cases" << endl;
        int T = time(NULL) - 1524576179;
        double PT = T / 60.0, TT = 75.0;
        cout << "Time   : " << T / 60 << " minutes " << T % 60 << " secs" << endl;
        cout << "Score  : " << 600 * (0.3 + (0.7 * TT * TT) / (10.0 * PT * PT + TT * TT)) << " points" << endl;
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
        cout << "CyclesNumber (600 Points)" << endl << endl;
    }
    return run_test(mainProcess, cases, argv[0]);
}
// CUT end
