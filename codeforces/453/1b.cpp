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

// This solution was written after the author read the editorial
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n + 1, 0);
  VI b(n + 1, 0);
  a[0] = 1;
  REP(i, 0, n) {
    REP(j, 0, n) {
      b[j + 1] += a[j];
      b[j + 1] %= 2;
    }
    swap(a, b);
  }
  cout << n << "\n";
  REP(i, 0, n + 1) {
    cout << a[i] << (i == n ? "\n" : " ");
  }
  cout << n - 1 << "\n";
  REP(i, 0, n) {
    cout << b[i] << (i == n - 1 ? "\n" : " ");
  }
}
