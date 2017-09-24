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



int main(void) {
  int n;
  cin >> n;
  VI a(n);
  int f4 = 0;
  int f2 = 0;
  int f1 = 0;
  REP(i, 0, n) {
    cin >> a[i];
    if (a[i] % 4 == 0) {
      f4 += 1;
    } else if (a[i] % 2 == 0) {
      f2 += 1;
    } else {
      f1 += 1;
    }
  }
  if (f2 >= 1) {
    f2 = 0;
    f1 += 1;
  }
  cout << (f4 < f1 - 1 ? "No" : "Yes") << endl;
}
