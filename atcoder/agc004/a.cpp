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
  ll a[3];
  cin >> a[0] >> a[1] >> a[2];
  sort(a, a + 3);
  if (a[0] % 2 == 0 || a[1] % 2 == 0 || a[2] % 2 == 0) {
    cout << 0 << endl;
    return 0;
  }
  cout << a[0] * a[1] << endl;
}
