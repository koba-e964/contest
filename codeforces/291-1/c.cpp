#include <vector>
#include <string>

class HugeMod {
public:
  static const int B = 3;
  static const long long mods[B];
  typedef long long ll;
private:
  ll cont[B];
public:
  HugeMod(void): cont() {
    for(int i = 0; i < B; ++i) {
      cont[i] = 0;
    }
  }
  HugeMod(ll x) {
    for (int i = 0; i < B; ++i) {
      ll y = x % mods[i];
      y += mods[i];
      y %= mods[i];
      cont[i] = y;
    }
  }
  HugeMod(const HugeMod &other) {
    for (int i = 0; i < B; ++i) {
      cont[i] = other.cont[i];
    }
  }
  HugeMod operator+(const HugeMod &o) const {
    HugeMod hm;
    for (int i = 0; i < B; ++i) {
      hm.cont[i] = (cont[i] + o.cont[i]) % mods[i];
    }
    return hm;
  }
  HugeMod operator-(const HugeMod &o) const {
    HugeMod hm;
    for (int i = 0; i < B; ++i) {
      hm.cont[i] = (cont[i] - o.cont[i] + mods[i]) % mods[i];
    }
    return hm;
  }
  HugeMod operator*(const HugeMod &o) const {
    HugeMod hm;
    for (int i = 0; i < B; ++i) {
      hm.cont[i] = (cont[i] * o.cont[i]) % mods[i];
    }
    return hm;    
  }
  void operator+=(const HugeMod &o) {
    for (int i = 0; i < B; ++i) {
      cont[i] = (cont[i] + o.cont[i]) % mods[i];
    }
  }
  void operator-=(const HugeMod &o) {
    for (int i = 0; i < B; ++i) {
      cont[i] = (cont[i] - o.cont[i] + mods[i]) % mods[i];
    }
  }
  void operator*=(const HugeMod &o) {
    for (int i = 0; i < B; ++i) {
      cont[i] = (cont[i] * o.cont[i]) % mods[i];
    }
  }
  HugeMod pow(int val) const {
    HugeMod sum = 1;
    HugeMod cur = *this;
    while (val > 0) {
      if (val % 2) {
	sum *= cur;
      }
      cur *= cur;
    }
    return sum;
  }
  bool operator == (const HugeMod &o) const {
    for (int i = 0; i < B; ++i) {
      if (cont[i] != o.cont[i]) {
	return false;
      }
    }
    return true;
  }
  bool operator<(const HugeMod &o) const {
    for (int i = 0; i < B; ++i) {
      if (cont[i] != o.cont[i]) {
	return cont[i] < o.cont[i];
      }
    }
    return false;
  }
};
const long long HugeMod::mods[HugeMod::B] = {1040000123LL, 1040000141LL, 1040000173LL};
HugeMod operator*(long long val, const HugeMod &h) { return h * val;}

/*
 * Header requirement: vector, string
 */
template<class Value>
class RollingHash {
public:
private:
  Value base;
  std::vector<Value> pw;
  static const int L = 700010;
  void pw_init(void) {
    Value tmp = 1;
    for (int i = 0; i < L; ++i) {
      pw[i] = tmp;
      tmp *= base;
    }
  }
public:
  RollingHash(Value base) : base(base), pw(L) {
    pw_init();
  }
  Value hash(const std::string &s) {
    int n = s.length();
    Value sum = 0;
    for (int i = 0; i < n; ++i) {
      sum *= base;
      sum += s[i];
    }
    return sum;
  }
  Value &pow_table(int i) {
    return pw.at(i);
  }
};
#include <iostream>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef unsigned long long int ull;


vector<HugeMod> mech;

int main(void){
  int n, m;
  cin >> n >> m;
  RollingHash<HugeMod> rh(7);
  REP(i, 0, n) {
    string s;
    cin >> s;
    HugeMod res = rh.hash(s);
    mech.push_back(res);
  }
  sort(mech.begin(), mech.end());
  REP(loop_var, 0, m) {
    string s;
    cin >> s;
    int l = s.length();
    HugeMod h = rh.hash(s);
    bool ok = 0;
    REP(i, 0, l) {
      REP(ch, 'a', 'd') {
	ll oc = (int)ch - (int)s[i]; 
	if (oc == 0) {
	  continue;
	}
	HugeMod newhash = h + oc * rh.pow_table(l - i - 1);
	if (*lower_bound(mech.begin(), mech.end(), newhash) == newhash) {
	  ok = 1;
	}
      }
    }
    cout << (ok ? "YES" : "NO") << endl;
  }
}
