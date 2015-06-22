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
      val /= 2;
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
  bool operator != (const HugeMod &o) const {
    return !(*this == o);
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
public:
  RollingHash(Value base) : base(base) {}
  Value hash(const std::string &s) {
    int n = s.length();
    Value sum = 0;
    for (int i = 0; i < n; ++i) {
      sum *= base;
      sum += s[i];
    }
    return sum;
  }
  /*
   * returns [hash(s[0 ... l]) | l = 0, ... , n]
   * hash("") = 0
   */
  std::vector<Value> hashes(const std::string &s) {
    int n = s.length();
    std::vector<Value> res(n + 1);
    Value sum = 0;
    for (int i = 0; i < n; ++i) {
      res[i] = sum;
      sum *= base;
      sum += s[i];
    }
    res[n] = sum;
    return res;
  }
};
#include <iostream>
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 1001000;
const ll mod = 1e9 + 7;
int a[N];

int tbl[N] = {};



int main(void){
  int n, m;
  string p;
  cin >> n >> m >> p;
  RollingHash<HugeMod> rh(7);
  vector<HugeMod> hashes = rh.hashes(p);
  REP(i, 0, m) {
    cin >> a[i];
    a[i]--;
    tbl[a[i]]++;
    tbl[a[i] + p.length()]--;
  }
  REP(i, 0, N - 1) {
    tbl[i + 1] += tbl[i];
  }
  ll sum = 1;
  REP(i, 0, n) {
    if(tbl[i] == 0) {
      sum *= 26;
      sum %= mod;
    }
  }
  int pl = p.length();
  REP(i, 0, m - 1) {
    int d = a[i + 1] - a[i];
    if (d >= pl) {
      continue;
    }
    // check whether p[d ... |p|] == p[0 ... |p| - d]
    if (hashes[pl - d] != hashes[pl] - hashes[d] * HugeMod(7).pow(pl - d)) {
      sum = 0;
    }
  }
  cout << sum << endl;
}
