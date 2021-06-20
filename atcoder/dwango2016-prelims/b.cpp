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
typedef pair<int, int> PI;
const double EPS=1e-9;



int main(void){
  int n;
  cin >> n;
  VI a(n - 1), b(n);
  REP(i, 0, n - 1) {
    cin >> a[i];
  }
  b[0] = a[0];
  b[n - 1] = a[n - 2];
  REP(i, 1, n - 1) {
    b[i] = min(a[i - 1], a[i]);
  }
  // check
  REP(i, 0, n - 1) {
    if (a[i] != max(b[i], b[i + 1])) {
      assert (!"error");
    }
  }
  REP(i, 0, n) {
    cout << b[i] << (i == n - 1 ? "\n" : " ");
  }
}
