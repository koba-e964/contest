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
  int n, x;
  cin >> n >> x;

  VI w(n);
  REP(i, 0, n) {
    cin >> w[i];
  }
  int m = n / 2;

  VI former(1 << m);
  VI latter(1 << (n - m));
  REP(i, 0, 1 << m) {
    int sum = 0;
    REP(j, 0, m) {
      if (i & (1 << j)) sum += w[j];
    }
    former[i] = sum;
  }

  REP(i, 0, 1 << (n - m)) {
    int sum = 0;
    REP(j, 0, n - m) {
      if (i & (1 << j)) sum += w[m + j];
    }
    latter[i] = sum;
  }

  sort(former.begin(), former.end());
  int cnt = 0;
  REP(bits, 0, 1 << (n - m)) {
    int rem = x - latter[bits];
    cnt += upper_bound(former.begin(), former.end(), rem) - lower_bound(former.begin(), former.end(), rem);
  }
  cout << cnt << endl;
}
