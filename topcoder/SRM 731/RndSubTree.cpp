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

#define REP(i, s, n) for (int i=(int)(s); i < (int)(n); ++i)

using namespace std;
typedef long long ll;
const ll mod = 1e9+7;

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

const int N = 2100;

class RndSubTree {
public:
  ll comb[N][N];
  void init(void) {
    comb[0][0] = 1;
    REP(i, 1, N) {
      REP(j, 0, N) {
	ll tmp = comb[i - 1][j];
	if (j > 0) tmp = (tmp + comb[i - 1][j - 1]) % mod;
	comb[i][j] = tmp;
      }
    }
  }
  void add(ll &x, ll y) {
    x = (x + y) % mod;
  }
  ll dp[N][2];
  int count(int k) {
    init();
    REP(i, 0, 2) dp[0][i] = 0;
    REP(i, 1, k + 1) {
      ll fac = powmod(2, mod - 1 - i + 1);
      REP(j, 0, 2) {
	ll tot = 0;
	REP(u, 0, i) {
	  ll tmp = 0;
	  add(tmp, dp[u][0]);
	  add(tmp, dp[u][1] * (j + i - u));
	  add(tmp, dp[i - u - 1][0]);
	  add(tmp, dp[i - u - 1][1] * (j + u + 1));
	  add(tmp, (ll)(2 * u + 1) * i);
	  add(tmp, mod - (2 * u * (ll)(u + 1) + 1));
	  add(tmp, j * (i - 1));
	  tmp = tmp * comb[i - 1][u] % mod;
	  add(tot, tmp);
	}
        tot = tot * fac % mod;
	if (j == 0) dp[i][0] = tot;
	else dp[i][1] = (tot - dp[i][0] + mod) % mod;
      }
      if (0) {
	cerr << dp[i][0] << " " << dp[i][1] << endl;
      }
    }
    return dp[k][0];
  }
};

// CUT begin
ifstream data("RndSubTree.sample");

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

template <typename T>
string to_string(T t) {
    stringstream s;
    s << t;
    return s.str();
}

string to_string(string t) {
    return "\"" + t + "\"";
}

bool do_test(int k, int __expected) {
    time_t startClock = clock();
    RndSubTree *instance = new RndSubTree();
    int __result = instance->count(k);
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
        int k;
        from_stream(k);
        next_line();
        int __answer;
        from_stream(__answer);

        cases++;
        if (case_set.size() > 0 && case_set.find(cases - 1) == case_set.end())
            continue;

        cout << "  Testcase #" << cases - 1 << " ... ";
        if ( do_test(k, __answer)) {
            passed++;
        }
    }
    if (mainProcess) {
        cout << endl << "Passed : " << passed << "/" << cases << " cases" << endl;
        int T = time(NULL) - 1521303934;
        double PT = T / 60.0, TT = 75.0;
        cout << "Time   : " << T / 60 << " minutes " << T % 60 << " secs" << endl;
        cout << "Score  : " << 500 * (0.3 + (0.7 * TT * TT) / (10.0 * PT * PT + TT * TT)) << " points" << endl;
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
        cout << "RndSubTree (500 Points)" << endl << endl;
    }
    return run_test(mainProcess, cases, argv[0]);
}
// CUT end
