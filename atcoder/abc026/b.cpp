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

double r[1000];

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    cin >> r[i];
  }
  sort(r, r + n);
  double sum = 0;
  for (int i = n - 1; i >= 1; i -= 2) {
    sum += r[i] * r[i] - r[i - 1] * r[i - 1];
  }
  if (n % 2 == 1) {
    sum += r[0] * r[0];
  }
  printf("%.9f\n", sum * acos(-1));
}
