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
  int n;
  cin >> n;
  VI a(n);
  int x = 0;
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
  }
  REP(i, 0, n) {
    if (a[i] == i) {
      if (i < n - 1) {
	swap(a[i], a[i + 1]);
	x += 1;
      } else {
	swap(a[n - 2], a[n - 1]);
	x += 1;
      }
    }
  }
  cout << x << endl;
}
