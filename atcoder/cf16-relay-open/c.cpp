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



int main(void){
  int n;
  cin >> n;
  VI a(1 << n);
  REP(i, 0, 1 << n) {
    cin >> a[i];
  }
  REP(k, 0, n) {
    VI b(1 << (n - k - 1));
    REP(bits, 0, 1 << (n - k - 1)) {
      int x = a[2 * bits];
      int y = a[2 * bits + 1];
      if (x > y) {
	swap(x, y);
      }
      if (x < y) {
	b[bits] = y - x;
      } else {
	b[bits] = x;
      }
    }
    a = b;
  }
  cout << a[0] << endl;
}
