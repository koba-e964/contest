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


int main(void){
  ll l1, r1, l2, r2, k;
  cin >> l1 >> r1 >> l2 >> r2 >> k;
  ll uv = max(l1, l2);
  ll tk = min(r1, r2);
  if (uv > tk) {
    cout << 0 << endl;
    return 0;
  }
  ll tmp = tk - uv + 1;
  if (uv <= k && k <= tk) {
    tmp -= 1;
  }
  cout << tmp << endl;
}
