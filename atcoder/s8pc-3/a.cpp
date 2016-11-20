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
  ll n;
  int k;
  cin >> n >> k;
  if (n <= 2) {
    cout << 0 << endl;
    return 0;
  }
  n -= 2;
  ll cnt11 = 0;
  REP(i, 0, 11) {
    REP(j, 1, 6) {
      if ((9 * (7 * i + j + 8)) % 11 == k) { cnt11++; }
    }
  }
  ll sum = 0;
  REP(i, 0, n % 11) {
    REP(j, 1, 6) {
      if ((9 * (7 * i + j + 8)) % 11 == k) { sum++; }
    }
  }
  cout << cnt11 * (n / 11) + sum << endl;
}
