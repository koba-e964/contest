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

void op(int k, int bias) {
  REP(i, 1, k) {
    int a = ((i - 1) ^ 1) + 1;
    REP(j, i + 1, k + 3) {
      if (j != a) {
	cout << i +bias << " " << j +bias << endl;
      }
    }
  }
  cout << k +bias << " " << k + 1 +bias << endl;
}

int main(void){
  int k;
  cin >> k;
  if (k % 2 == 0) {
    cout << "NO" << endl;
    return 0;
  }
  if (k == 1) {
    cout << "YES" << endl;
    cout << "2 1" << endl;
    cout << "1 2" << endl;
    return 0;
  }
  int v = 2 * k + 4;
  int e = k * k + 2 * k;
  cout << "YES" << endl << v << " " << e << endl;
  op(k, 0);
  cout << k + 2 << " " << v << endl;
  op(k, k + 2);
}
