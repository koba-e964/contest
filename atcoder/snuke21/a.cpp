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
  ll n;
  cin >> n;
  ll as = sqrt((double)n * 2.0);
  REP(i, max(1LL, as - (ll)1e5), min(as + (ll)1e5, 0x7fffffffLL)) {
    ll li = i;
    if (li * (li + 1) == n * 2) {
      cout << li << endl;
      return 0;
    }
  }
  cout << -1 << endl;
}
