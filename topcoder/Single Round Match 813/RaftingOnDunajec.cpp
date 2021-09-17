#include <cstdio>
#include <cmath>
#include <cstring>
#include <ctime>
#include <cassert>
#include <iostream>
#include <algorithm>
#include <set>
#include <sstream>
#include <vector>
#include <typeinfo>
#include <fstream>

using namespace std;
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

typedef long long int ll;

/*
 * Dependencies: typedef long long ll
 * Headers: iostream
 * Verified by: ARC099-F
 *              https://beta.atcoder.jp/contests/arc099/submissions/2743855
 */

template<ll mod = (ll)1e9 + 7>
struct ModInt {
  ll x;
  ModInt(void): x(0) {}
  ModInt(ll x): x(x % mod){}
  ModInt(const ModInt &x): x(x.x) {}
  ModInt operator+(ModInt o) const {
    ll y = x + o.x;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator-(ModInt o) const {
    ll y = x - o.x + mod;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator*(ModInt o) const {
    return ModInt((x * o.x) % mod);
  }
  void operator+=(ModInt o) { *this = *this + o; }
  void operator-=(ModInt o) { *this = *this - o; }
  void operator*=(ModInt o) { *this = *this * o; }
  ModInt operator-(void) const { return ModInt() - *this; }
  ll to_ll() const {
    return x;
  }
  bool operator<(ModInt o) const {
    return x < o.x;
  }
  ModInt pow(ll e) {
    assert (e >= 0);
    ModInt sum = 1;
    ModInt cur = *this;
    while (e > 0) {
      if (e % 2) {
        sum = sum * cur;
      }
      cur = cur * cur;
      e /= 2;
    }
    return sum;
  }
  ModInt inv(void) {
    return pow(mod - 2);
  }
};

template<ll mod>
ostream &operator<<(ostream &os, ModInt<mod> mi) {
  return os << mi.x;
}

class RaftingOnDunajec {
public:
  int count(int s, int c) {
    int n = 300;
    vector<ModInt<>> fac(n), invfac(n);
    fac[0] = 1;
    REP(i, 1, n) fac[i] = fac[i - 1] * i;
    REP(i, 0, n) invfac[i] = fac[i].inv();
    typedef vector<ModInt<>> vm;
    vector<vm> tbl(s + 1, vm(c + 1));
    REP(i, 0, s + 1) {
      REP(j, 0, c + 1) tbl[i][j] = ModInt<>(i * (i + 1) / 2).pow(j);
    }
    vector<vm> dp(s + 2, vm(c + 1));
    dp[0][0] -= 1;
    REP(i, 0, s + 1) {
      REP(j, i + 1, s + 2) {
        int len = j - i - 1;
        REP(l, 0, c + 1) {
          REP(u, 0, c - l + 1) {
            dp[j][l + u] -= dp[i][l] * fac[c - l] * invfac[u] * invfac[c - l - u] * tbl[len][u];
          }
        }
      }
    }
    return dp[s + 1][c].x;
  }
};

// CUT begin
ifstream data("RaftingOnDunajec.sample");

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

bool do_test(int S, int C, int __expected) {
    time_t startClock = clock();
    RaftingOnDunajec *instance = new RaftingOnDunajec();
    int __result = instance->count(S, C);
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
        int S;
        from_stream(S);
        int C;
        from_stream(C);
        next_line();
        int __answer;
        from_stream(__answer);

        cases++;
        if (case_set.size() > 0 && case_set.find(cases - 1) == case_set.end())
            continue;

        cout << "  Testcase #" << cases - 1 << " ... ";
        if ( do_test(S, C, __answer)) {
            passed++;
        }
    }
    if (mainProcess) {
        cout << endl << "Passed : " << passed << "/" << cases << " cases" << endl;
        int T = time(NULL) - 1631891717;
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
        cout << "RaftingOnDunajec (600 Points)" << endl << endl;
    }
    return run_test(mainProcess, cases, argv[0]);
}
// CUT end
