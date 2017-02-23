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
  if (n <= 2) {
    cout << 1 << endl;
    REP(i, 0, n) {
      cout << 1 << (i == n - 1 ? "\n" : " ");
    }
    return 0;
  }
  VI prime(n + 2, 1);
  prime[0] = 0;
  prime[1] = 0;
  for (int i = 2; i * i <= n + 1; ++i) {
    for (int j = 2; i * j <= n + 1; ++j) {
      prime[i * j] = false;
    }
  }
  cout << 2 << endl;
  REP(i, 0, n) {
    cout << prime[i + 2] + 1 << (i == n - 1 ? "\n" : " ");
  }
}
