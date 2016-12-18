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
  VI a(n);
  VI sol(n);
  REP(i, 0, n) {
    int b;
    cin >> b;
    a[b]++;
  }
  ll tot = 1;
  REP(i, 0, n / 2) {
    sol[n  - 2*i-1] = 2;
  }
  if (n % 2==1) {
    sol[0] = 1;
  }
  REP(i, 0, n) {
    if (a[i] == sol[i]) {
      tot *= a[i] == 2 ? 2 : 1;
      tot %= mod;
    } else {
      tot = 0;
    }
  }
  cout << tot << endl;
}
