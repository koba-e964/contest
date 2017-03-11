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

int main(void){
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 1, 2 * n - 3) {
    REP(p, 0, (i + 1) / 2) {
      int q = i - p;
      if (q >= n) continue;
      assert (p < q);
      if (a[p] > a[q]) {
	swap(a[p], a[q]);
      }
    }
  }
  REP(i, 0, n) {
    cout << a[i] << (i == n - 1 ? "\n" : " ");
  }
}
