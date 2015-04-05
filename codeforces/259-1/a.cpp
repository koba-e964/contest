#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;



int m,n;
int main(void){
	cin>>m>>n;
	double t=(double)1 / (double)m;
	double sum=0;
	REP(i,1,m+1) {
		sum += i* pow((double)i/double(m),n);
		sum -= i* pow((double)(i-1)/double(m), n);
	}
	printf("%.10f\n", sum);
}
