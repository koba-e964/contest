#include <cstdio>
#include <cassert>
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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

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

int gcd(int x, int y) {
  while (y) {
    int r = x % y;
    x = y; y = r;
  }
  return x;
}

class ClassifyQuads {
public:
  vector<int> classify(int n, int m) {
    vector<VI> gcds(3001, VI(3001, 0));
    REP(j, 0, 3001) gcds[0][j] = j;
    REP(i, 1, 3001) {
      REP(j, 0, 3001) gcds[i][j] = gcds[j % i][i];
    }
    vector<ModInt<>> ans(6);
    VI ret(6, 0);
    ModInt<> nm(n * m);
    ModInt<> inv2 = ModInt<>(2).inv();
    ModInt<> inv3 = ModInt<>(3).inv();
    ModInt<> inv24 = ModInt<>(24).inv();
    ModInt<> remain = nm * (nm - 1) * (nm - 2) * (nm - 3) * inv24;
    int ss = 0;
    REP(dx, 0, n / 2 + 1) {
      REP(dy, 0, m / 2 + 1) {
        int g = gcds[dx][dy];
        if (g != 1) continue;
        int lim = 10000;
        if (dx) lim = min(lim, n / dx + 1);
        if (dy) lim = min(lim, m / dy + 1);
        vector<int> cnt(lim + 2);
        REP(xx, 1, lim + 1) {
          int lx = n - (xx - 1) * dx;
          int ly = m - (xx - 1) * dy;
          if (lx > 0 && ly > 0) {
            cnt[xx] = lx * ly;
            if (dx && !dy && lx >= dx) {
              cnt[xx] -= (lx - dx) * ly;
            }
            if (!dx && dy && ly >= dy) {
              cnt[xx] -= lx * (ly - dy);
            }
            if (dx && dy && lx >= dx && ly >= dy) {
              cnt[xx] -= (lx - dx) * (ly - dy);
            }
          }
          // cout << "dx = " << dx << " dy = " << dy << " cnt[" << xx << "] = " << cnt[xx] << endl;
        }
        REP(xx, 3, lim + 1) {
          ModInt<> val(xx);
          int coef = dx == 0 || dy == 0 ? 1 : 2;
          ans[0] += (val - 2) * (val - 3) * (cnt[xx] - cnt[xx + 1]) * coef;
          remain -= val * (val - 1) * (val - 2) * (val - 3)
            * (cnt[xx] - cnt[xx + 1]) * inv24 * coef;
          ans[1] -= val * (val - 1) * (val - 2)
            * (cnt[xx] - cnt[xx + 1]) * inv2 * inv3 * coef;
          ss++;
        }
        REP(xx, 3, lim + 1) {
          ModInt<> val(xx);
          // if dx != 0 && dy != 0, we need the (dx, -dy) direction
          int coef = dx == 0 || dy == 0 ? 1 : 2;
          ans[2] += val * (val - 1) * (val - 2) * (cnt[xx] - cnt[xx + 1])
            * (n * m - xx) * coef;
          remain -= val * (val - 1) * (val - 2) * (cnt[xx] - cnt[xx + 1])
            * (n * m - xx) * coef * inv2 * inv3;
          ss++;
        }
        // cout << "dx = " << dx << " dy = " << dy << " " << ans[2] << endl;
      }
    }
    ans[1] += nm * (nm - 1) * (nm - 2) * inv2 * inv3;
    REP(i, 0, n) {
      REP(j, 0, m) {
        if (gcds[i][j] != 1) continue;
        REP(k, i, n) {
          REP(l, 0, m) {
            if (gcds[k][l] != 1) continue;
            if (gcds[k - i][abs(l - j)] != 1) continue;
            if (i * m + j >= k * m + l) continue;
            if (i * l == k * j) continue;
            int coef = l >= j ? 2 : 4;
            if ((i == 0 && l == j) || (i == k && j == 0)) {
              coef = 1;
            } else if (i == 0 || j == 0 || l == 0 || i == k || j == l) {
              coef = 2;
            }
            int lx = n - k, ly = m - max(j, l);
            ans[1] -= lx * ly * coef;
          }
        }
      }
    }
    REP(i, 0, n) {
      REP(j, 0, m) {
        REP(k, i, n) {
          REP(l, 0, m) {
            if (i * m + j >= k * m + l) continue;
            if (i * l == k * j) continue;
            int coef = l >= j ? 2 : 4;
            if ((i == 0 && l == j) || (i == k && j == 0)) {
              coef = 1;
            } else if (i == 0 || j == 0 || l == 0 || i == k || j == l) {
              coef = 2;
            }
            int lx = n - k, ly = m - max(j, l);
            // Pick's theorem
            int area2 = abs(i * l - j * k);
            int perim = 0;
            perim += gcds[i][j];
            perim += gcds[k][l];
            perim += gcds[k - i][abs(l - j)];
            int latp = (area2 + 2 - perim) / 2;
            ans[3] += lx * ly * latp * coef * 3;
          }
        }
      }
    }
    remain -= ans[3] * inv3;
    cout << "ss = " << ss << endl;
    ans[0] *= inv2;
    ans[2] *= inv3;
    ans[4] = remain;
    ans[5] = remain * 2;
    REP(i, 0, 6) {
      ret[i] = ans[i].to_ll();
    }
    return ret;
  }
};

// CUT begin
ifstream data("ClassifyQuads.sample");

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

bool do_test(int R, int C, vector<int> __expected) {
    time_t startClock = clock();
    ClassifyQuads *instance = new ClassifyQuads();
    vector<int> __result = instance->classify(R, C);
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
        int R;
        from_stream(R);
        int C;
        from_stream(C);
        next_line();
        vector<int> __answer;
        from_stream(__answer);

        cases++;
        if (case_set.size() > 0 && case_set.find(cases - 1) == case_set.end())
            continue;

        cout << "  Testcase #" << cases - 1 << " ... ";
        if ( do_test(R, C, __answer)) {
            passed++;
        }
    }
    if (mainProcess) {
        cout << endl << "Passed : " << passed << "/" << cases << " cases" << endl;
        int T = time(NULL) - 1630546358;
        double PT = T / 60.0, TT = 75.0;
        cout << "Time   : " << T / 60 << " minutes " << T % 60 << " secs" << endl;
        cout << "Score  : " << 1000 * (0.3 + (0.7 * TT * TT) / (10.0 * PT * PT + TT * TT)) << " points" << endl;
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
        cout << "ClassifyQuads (1000 Points)" << endl << endl;
    }
    return run_test(mainProcess, cases, argv[0]);
}
// CUT end
