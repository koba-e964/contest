#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;

ll curtime=1535814000;
string name="Murik";

int pw[8] = {484, 467, 469, 363, 621, 504, 291, 503};
int gene[8] = {1, 3, 2, 3, 0, 0, 4, 2};

//http://math314.hateblo.jp/entry/2015/05/07/014908 からコピー && 改造
template<class T> T ext_gcd(T a, T b, T& x, T& y) { for (T u = y = 1, v = x = 0; a;) { T q = b / a; swap(x -= q * u, u); swap(y -= q * v, v); swap(b -= q * a, a); } return b; }
template<class T> T zmod(T a, T b) {
  a %= b;
  if (a < 0) a += b;
  return a;
}
template<class T> T invmod(T a, T m) {
  T x, y;
  ext_gcd(a, m, x, y);
  return zmod(x, m);
}

const ll A = 1234567893;
const ll B = 151515151;
const ll p31 = 1LL << 31;


int nxt(int x) {
  int y=A*x+B;
  return y&0x7fffffff;
}

PL mul(PL a, PL b) {
  ll u = a.first * b.first % p31;
  ll v = (a.first * b.second + a.second) % p31;
  return PL(u, v);
}

PL powmod(PL op, int e) {
  PL sum(1, 0);
  PL cur(op);
  while (e > 0) {
    if (e % 2) sum = mul(sum, cur);
    cur = mul(cur, cur);
    e /= 2;
  }
  return sum;
}

ll myhash(const string &name) {
  ll res = 0;
  for (char c: name) {
    res = 31 * res + (int) c;
    res &= p31 - 1;
  }
  return res;
}

void salt_guess(void) {
  VI cand;
  for (ll i=1;i<1LL<<31;i+=5){
    ll v=i;
    bool ok=true;
    REP(j, 1, 8) {
      v=nxt(v);
      if (v%5!=gene[j]){
        ok=false;
        break;
      }
    }
    if(ok)cand.push_back(i);
  }
  ll IA = invmod(A, p31);
  ll IB = (p31 - B) * IA % p31;
  cerr << IA << " * y + " << IB << endl;
  PL I(IA, IB);
  PL J = powmod(I, 80000);
  ll initseed = -1;
  REP(i, 0, cand.size()) {
    int c = cand[i];
    ll orig = (J.first * c + J.second) % p31;
    bool ok = true;
    ll v = orig;
    REP(j, 0, 8) {
      int sum = 500;
      REP(k, 0, 10000) {
        sum += (v % 3) - 1;
        v = nxt(v);
      }
      if (sum != pw[j]) {
        ok = false;
        break;
      }
    }
    if (ok) {
      initseed = orig;
    }
  }
  initseed = (IA * initseed + IB) % p31;
  cout << "ll initseed = " << initseed << endl;
  ll salt = initseed ^ myhash(name) ^ curtime;
  cout << "ll salt = " << salt << endl;
}

ll salt = 1748665573;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  //salt_guess();
  ll start, wait;
  string name;
  cin >> start >> wait >> name;
  ll hname = myhash(name);
  vector<VI> kk;
  PI ma(-1, -1);
  for (ll t = start; t <= start + wait; ++t) {
    ll seed = t ^ hname ^ salt;
    VI pwgene(16);
    ll v = seed;
    REP(j, 0, 8) {
      int sum = 500;
      REP(k, 0, 10000) {
        v = nxt(v);
        sum += (v % 3) - 1;
      }
      pwgene[j] = sum;
    }
    REP(j, 0, 8) {
      v = nxt(v);
      pwgene[8 + j] = v % 5;
    }
    int score = 0;
    REP(j, 0, 8) {
      score += abs(pwgene[j] - 500);
    }
    kk.push_back(pwgene);
    ma = max(ma, PI(score, -t));
  }
  cout << -ma.second << " " << setprecision(15) << ma.first / 8.0 << endl;
  int idx = -ma.second - start;
  REP(j, 0, 8) {
    cout << kk[idx][j] << (j == 7 ? "\n" : " ");
  }
  REP(j, 0, 8) {
    cout << kk[idx][8 + j] << (j == 7 ? "\n" : " ");
  }
}
