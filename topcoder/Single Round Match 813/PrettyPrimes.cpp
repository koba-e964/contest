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

using namespace std;
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

typedef long long int ll;
typedef vector<ll> VL;

bool is_prime(ll x) {
  if (x <= 1) return 0;
  ll i = 2;
  while (i * i <= x) {
    if (x % i == 0) return 0;
    i++;
  }
  return 1;
}

void dfs1(int pat, ll s, int marg, int d, VL &ans) {
  if (marg > d) return;
  if (d == 0) {
    if (is_prime(s)) ans.push_back(s);
    return;
  }
  dfs1(pat, 10 * s + pat, marg, d - 1, ans);
  if (marg > 0) {
    REP(i, s == 0 ? 1 : 0, 10) {
      if (i != pat) {
        dfs1(pat, 10 * s + i, marg - 1, d - 1, ans);
      }
    }
  }
}

void dfs11(int pat, ll s, int marg, int d, VL &ans) {
  if (marg > d) return;
  if (d == 0) {
    if (is_prime(s)) ans.push_back(s);
    return;
  }
  dfs11(pat, 10 * s + pat, marg, d - 1, ans);
  int last = s % 10;
  int del = s != 0 && last == pat ? 2 : 1;
  if (d == 1) del--;
  if (marg >= del) {
    REP(i, s == 0 ? 1 : 0, 10) {
      if (i != pat) {
        dfs11(pat, 10 * s + i, marg - del, d - 1, ans);
      }
    }
  }
}

void dfs2(int pat, ll s, int marg, int d, VL &ans) {
  if (marg > d) return;
  if (d == 0) {
    if (is_prime(s)) ans.push_back(s);
    return;
  }
  if (d >= 2) {
    dfs2(pat, 100 * s + pat, marg, d - 2, ans);
  }
  if (marg > 0) {
    REP(i, s == 0 ? 1 : 0, 10) {
      dfs2(pat, 10 * s + i, marg - 1, d - 1, ans);
    }
  }
}

const ll mod = 1e9 + 7;

bool debug = 0;

class PrettyPrimes {
public:
  int solve(int pat, int d) {
    ll tot = 0;
    if (pat < 10) {
      VL ans;
      int marg = 0;
      while (1) {
        dfs1(pat, 0, marg, d, ans);
        if (ans.size()) break;
        marg++;
      }
      if (debug) for (ll x: ans) cout << x << endl;
      for (ll x: ans) tot += x % mod;
      return tot % mod;
    }
    if (pat % 11 == 0) {
      VL ans;
      int marg = 0;
      while (1) {
        dfs11(pat / 11, 0, marg, d, ans);
        if (ans.size()) break;
        marg++;
      }
      if (debug) for (ll x: ans) cout << x << endl;
      for (ll x: ans) tot += x % mod;
      return tot % mod;
    }
    VL ans;
    int marg = 0;
    while (1) {
      dfs2(pat, 0, marg, d, ans);
      if (ans.size()) break;
      marg++;
    }
    if (debug) for (ll x: ans) cout << x << endl;
    for (ll x: ans) tot += x % mod;
    return tot % mod;
    return 0;
  }
};

// CUT begin
ifstream data("PrettyPrimes.sample");

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

bool do_test(int pattern, int D, int __expected) {
    time_t startClock = clock();
    PrettyPrimes *instance = new PrettyPrimes();
    int __result = instance->solve(pattern, D);
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
        int pattern;
        from_stream(pattern);
        int D;
        from_stream(D);
        next_line();
        int __answer;
        from_stream(__answer);

        cases++;
        if (case_set.size() > 0 && case_set.find(cases - 1) == case_set.end())
            continue;

        cout << "  Testcase #" << cases - 1 << " ... ";
        if ( do_test(pattern, D, __answer)) {
            passed++;
        }
    }
    if (mainProcess) {
        cout << endl << "Passed : " << passed << "/" << cases << " cases" << endl;
        int T = time(NULL) - 1631893028;
        double PT = T / 60.0, TT = 75.0;
        cout << "Time   : " << T / 60 << " minutes " << T % 60 << " secs" << endl;
        cout << "Score  : " << 900 * (0.3 + (0.7 * TT * TT) / (10.0 * PT * PT + TT * TT)) << " points" << endl;
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
        cout << "PrettyPrimes (900 Points)" << endl << endl;
    }
    return run_test(mainProcess, cases, argv[0]);
}
// CUT end
