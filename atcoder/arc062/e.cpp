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
const int DEBUG = 0;

const int N = 400;
int c[N][4];
ll hashes[N];

map<ll, int> dict;
map<ll, int> mult;

pair<ll, int> norm(ll hash) {
  ll mi = hash;
  map<ll, int> t;
  REP(j, 0, 4) {
    if (t.count(hash) == 0) {
      t[hash] = 0;
    }
    t[hash]++;
    mi = min(mi, hash);
    hash = hash * 1000 % 999999999999;
  }
  return pair<ll, int>(mi, t[hash]);
}

ll calc(const VI &cols, int j) { // calucates #ways to fill in the rest 4 squares in [i, n) - {j}
  ll jhash = hashes[j];
  ll prod = 1;
  map<ll, int> alt;
  REP(k, 0, 4) {
    ll hash = cols[k] + cols[4 + k] * 1000;
    hash += ll(cols[4 + ((1 + k) % 4)]) * 1000000;
    hash += ll(cols[(k + 1) % 4]) * 1000000000;
    hash = norm(hash).first;
    if (alt.count(hash) == 0) {
      alt[hash] = 0;
    }
    alt[hash]++;
  }
  for (map<ll, int>::iterator it = alt.begin(); it != alt.end(); ++it) {
    ll hash = it->first;
    int times = it->second;
    if (dict.count(hash) == 0) {
      return 0;
    }
    int numh = dict[hash];
    if (DEBUG) { cerr << "original numh = " << numh << endl; }
    if (jhash == hash) {
      numh--;
    }
    assert (numh >= 0);
    ll tmp = 1;
    REP(k, 0, times) {
      tmp *= numh - k;
      tmp *= mult[hash];
    }
    prod *= tmp;
    if (DEBUG) { cerr << "C(" << numh << "," << times << ") * mult^times=" << tmp << endl; }
  }
  return prod;
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    REP(j, 0, 4) {
      cin >> c[i][j];
    }
  }
  ll sum = 0;
  for (int i = n - 1; i >= 0; --i) { // the top facet is fixed (including its direction)
    if (i < n - 5) {
      REP(j, i + 1, n) { // the bottom facet is fixed
	if (DEBUG) {
	  cerr << "(i, j) = " << i << ", " << j << endl;
	}
	REP(k, 0, 4) { // the direction of the bottom facet is fixed
	  if (DEBUG) { cerr << "rotation " << k << endl; }
	  VI cols(8);
	  REP(l, 0, 4) {
	    cols[l] = c[i][l];
	  }
	  REP(l, 0, 4) {
	    cols[7 - l] = c[j][(l + k) % 4];
	  }
	  sum += calc(cols, j);
	  if (DEBUG) { cerr << "sum = " << sum << endl; }
	}
      }
    }
    ll hash = 0; // c0 + 10^3 c1 + 10^6 c2 + 10^9 c3
    REP(j, 0, 4) {
      hash *= 1000;
      hash += c[i][3 - j];
    }
    pair<ll, int> normed = norm(hash);
    hashes[i] = hash = normed.first;
    if (dict.count(hash) == 0) {
      dict[hash] = 0;
    }
    dict[hash]++;
    mult[hash] = normed.second;
    if (DEBUG) { cerr << "added: " << hash << "[" << dict[hash] << "]" << endl;}

  }
  cout << sum << endl;
}
