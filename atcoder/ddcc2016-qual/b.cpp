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

int r, n, m;
double line(int i) {
  if (i <= 0 || i >= n) {
    return 0;
  }
  return 2 * r * sqrt(1.0 / 4 * n * n - pow(i - n / 2.0, 2)) * 2.0 / n;
    
}

int main(void){
  cin >> r >> n >> m;
  double sum = 0;
  REP(i, 1, n + m) {
    sum += max(line(i), line(i - m));
  }
  printf("%.9f\n", sum);
}
