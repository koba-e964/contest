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
  int n, k;
  cin >> n >> k;
  VI a(n);
  int sum = 0;
  REP(i, 0, n) {
    cin >> a[i];
    sum += a[i];
  }
  // sum + k * x >= (k - 1/2) * (n + x)
  // <=> sum >= (k - 1/2) * n - x / 2
  // <=> x >= (2k - 1) * n - 2 * sum
  cout << max(0, (2 * k - 1) * n - 2 * sum) << endl;
}
