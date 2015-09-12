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
  if (n == 2 || n == 3 || n == 5) {
    cout << "Prime" << endl;
    return 0;
  }
  if (n >= 2 && n % 2 != 0 && n % 3 != 0 && n % 5 != 0) {
    cout << "Prime" << endl;
  } else {
    cout << "Not Prime" << endl;
  }
}
