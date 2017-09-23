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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int calc(VI rest) {
  sort(rest.begin(), rest.end());
  int tot = 0;
  REP(i, 0, rest.size() / 2) {
    tot += rest[2 * i + 1] - rest[2 * i];
  }
  return tot;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(2 * n);
  REP(i, 0, 2 * n) {
    cin >> a[i];
  }
  int mi = 1e8;
  REP(i, 0, 2 * n) {
    REP(j, 0, i) {
      VI rest;
      REP(k, 0, 2 * n) {
	if (i != k && j != k) {
	  rest.push_back(a[k]);
	}
      }
      mi = min(mi, calc(rest));
    }
  }
  cout << mi << "\n";
}
