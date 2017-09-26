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
  int n;
  cin >> n;
  VI a(n);
  VI freq(n + 1, -1);
  REP(i, 0, n) {
    cin >> a[i];
    freq[a[i]] += 1;
  }
  map<int, int> more;
  queue<int> less;
  REP(i, 1, n + 1) {
    if (freq[i] == -1) {
      less.push(i);
    } else {
      REP(j, 0, freq[i]) {
	more[i] += 1;
      }
    }
  }
  int q = 0;
  REP(i, 0, n) {
    if (more[a[i]] > 0) {
      assert (less.size() > 0);
      int f = less.front();
      if (freq[a[i]] >= more[a[i]] && a[i] < f) {
	freq[a[i]] -= 1;
      } else {
	freq[a[i]] -= 1;
	more[a[i]] -= 1;
	q += 1;
	a[i] = f;
	less.pop();
      }
    } else {
      freq[a[i]] -= 1;
    }
  }
  cout << q << "\n";
  REP(i, 0, n) {
    cout << a[i] << (i == n - 1 ? "\n" : " ");
  }
}
