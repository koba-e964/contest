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
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n;
  int q;
  cin >> n >> q;
  REP(i , 0, q) {
    ll v, w;
    cin >> v >> w;
    if (n == 1) {
      cout << min(v, w) << "\n";
    } else {
      while (v != w) {
	if (v > w) {
	  swap(v, w);
	}
	w = (w + n - 2) / n;
      }
      cout << v << endl;
    }
  }
}
