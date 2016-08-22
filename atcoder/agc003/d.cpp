#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

map<ll, ll> opp_tbl;

vector<PI> inv(const vector<PI> &v) {
  vector<PI> opp(v.size());
  REP(i, 0, v.size()) {
    opp[i] = PI(v[i].first, 3 - v[i].second);
  }
  return opp;
}

VI primes;
void primes_init(void) {
  int s = 3001;
  VI tbl(s, 1);
  tbl[0] = tbl[1] = 0;
  REP(i, 2, s) {
    REP(j, 2, (s + i - 1) / i) {
      tbl[i * j] = 0;
    }
  }
  REP(i, 2, s) {
    if (tbl[i]) {
      primes.push_back(i);
    }
  }
}

/*
 * n |-> (r, (...)^3/r)
 * n / r = (...)^3
 * if n is not factored into a product of p <= 10^5, or the opposite is not in range, return (-1, ?).
 */
pair<ll, ll> opposite(ll v) {
  int oldv = v;
  ll f_ll = 1, opp_ll = 1;
  int e = 0;
  int ind = 0;
  while ((v >= 2 || e >= 1) && ind < primes.size()) {
    int p = primes[ind];
    if (v % p == 0) {
      v /= p;
      e++;
    } else {
      if (e % 3 != 0) {
	REP(j, 0, e % 3) {
	  f_ll *= p;
	}
	REP(j, 0, 3 - e % 3) {
	  opp_ll *= p;
	  if (opp_ll > 10000000000) {
	    return pair<ll, ll>(-1, -1);
	  }
	}
      }
      e = 0;
      ind++;
    }
  }
  // Is v square?
  ll sq = sqrt(v);
  if (v >= 2 && v == sq * sq) {
    f_ll *= sq * sq;
    opp_ll *= sq;
    if (opp_ll > 10000000000) {
      return pair<ll, ll>(-1, -1);
    }
    v = 1;
  }

  // if v <= 100000 (<= 3000^2) then v is prime and v^2 is in range
  if (v <= 100000) {
    f_ll *= v;
    REP(j, 0, 2) {
      opp_ll *= v;
      if (opp_ll > 10000000000) {
	return pair<ll, ll>(-1, -1);
      }
    }
    v = 1;
  }
  if (v >= 2) {
    return pair<ll, ll>(-1, -1);
  }
  opp_tbl[f_ll] = opp_ll;
  opp_tbl[opp_ll] = f_ll;
  //cerr << "old f opp:" << oldv << " " << f_ll << " " << opp_ll << endl;
  return pair<ll, ll>(f_ll, opp_ll);
}

int main(void){
  int n;
  cin >> n;
  VL s(n);
  REP(i, 0, n) {
    cin >> s[i];
  }
  primes_init();
  int cnt = 0;
  map<ll, int> tbl;
  REP(i, 0, n) {
    pair<ll, ll> norm = opposite(s[i]);
    if (norm.first == -1) {
      cnt++;
    } else {
      if (tbl.count(norm.first) == 0) {
	tbl[norm.first] = 1;
      } else {
	tbl[norm.first]++;
      }
    }
  }
  for (map<ll, int>::iterator it = tbl.begin(); it != tbl.end(); ++it) {
    pair<ll, int> w = *it;
    if (w.first == 1) {
      cnt++;
      continue;
    }
    ll opn = opp_tbl[w.first];
    int opc = tbl.count(opn) ? tbl[opn] : 0;
    cnt += max(w.second, opc);
    it->second = 0;
    tbl[opn] = 0;
  }
  cout << cnt << endl;
}
