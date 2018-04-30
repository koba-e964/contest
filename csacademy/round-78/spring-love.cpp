#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

int main(void) {
    int n;
    cin >> n;
    VI a(n);
    REP(i, 0, n) cin >> a[i];
    int sum = 0;
    REP(i, 0, n) sum += a[i];
    cout << sum % 2 << endl;
}
