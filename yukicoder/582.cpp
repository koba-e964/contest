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
  int tot = 0;
  REP(i, 0, n) {
    cin >> a[i];
    tot += a[i];
  }
  sort(a.begin(), a.end());
  if (tot % 2 == 1 && ((n >= 1 && a[n - 1] == 1) || (n >= 2 && a[n - 1] == 2 && a[n - 2] == 1))) {
    cout << "A\n";
  } else {
    cout << "B\n";
  }
}
