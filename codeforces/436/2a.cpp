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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n);
  VI freq(101, 0);
  REP(i, 0, n) {
    cin >> a[i];
    freq[a[i]] += 1;
  }
  vector<VI> f2(101, VI());
  REP(i, 0, 101) {
    if (freq[i] > 0) {
      f2[freq[i]].push_back(i);
    }
  }
  REP(i, 0, 101) {
    if (i * 2 == n && f2[i].size() >= 2) {
      cout << "YES\n";
      cout << f2[i][0] << " " << f2[i][1] << "\n";
      return 0;
    }
  }
  cout << "NO\n";
}
