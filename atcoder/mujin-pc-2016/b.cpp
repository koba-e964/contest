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
  double x, y, z;
  cin >> x >> y >> z;
  vector<double> v(3);
  v[0] = x;
  v[1] = y;
  v[2] = z;
  sort(v.begin(), v.end());
  double area = pow(x + y + z, 2) - pow(max(v[2] - v[1] - v[0], 0.0), 2);
  printf("%.15f\n", acos(-1) * area);
}
