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
  VI t(5, 0);
  REP(i, 0, n) {
    cin >> a[i];
    t[a[i]]++;
  }
  int tot = 0;
  tot += t[4];
  REP(i, 0, t[3]) {
    if (t[1] > 0) {
      t[1]--;
      tot++;
    } else {
      tot++;
    }
  }
  tot += (t[2] * 2 + t[1] + 3) / 4;
  cout << tot << endl;
}
