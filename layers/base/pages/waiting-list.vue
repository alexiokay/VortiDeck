<template lang="pug">

div(class="w-full h-full  -mt-[4rem] flex flex-col font-intern bg-white  pt-[2.5rem] md:pt-[4rem] lg:pt-[5rem] overflow-x-clip")
    div(class=" flex")
        div
            button(@click="setLocale('en')") en
            button(@click="setLocale('nl')") nl
            p {{ $t('message') }}
    div(class="flex flex-col w-full h-auto ")
        nuxt-img(src="images/logoFT.png" class="mx-auto" alt="logo" width="200" height="200")
    div(class="w-full h-auto lg:mt-[4rem] flex flex-col lg:flex-row  font-intern lg:px-[15rem] py-[3rem] gap-x-[2rem] xl:gap-x-[8rem] gap-y-[2rem] justify-center")
        div(class="w-auto flex flex-col gap-y-6 text-center lg:text-start px-6 lg:px-0")
        
            h1(class="font-bold text-3xl lg:text-4xl ") Join our WaitList.
            p(class="text-xl lg:text-3xl lg:w-[35rem] tracking-wider text-start ") Be the first in line to witness our groundbreaking platform for cleaning businesses.
            div(class="flex flex-col text-start items-start")
                p.mt-auto We are launching soon. 
                p.mt-1 Join our waiting list to be the first to know when we launch.
        
        form(class="w-full px-8 lg:px-0 lg:w-auto mx-auto lg:mx-0 flex flex-col gap-y-6 text-xl launchlist-form " action="https://getlaunchlist.com/s/sunvQN" method="POST")
    
                input(nname="name" class="w-full lg:w-[25rem] xl:w-[30rem] border-[#010302] border-[1px] px-4 py-2" placeholder="Your Name")
                input(name="email" class="w-full lg:w-[25rem] xl:w-[30rem]  border-[#010302] border-[1px] px-4 py-2" placeholder="Business Email address")
                input(name="location" class="w-full lg:w-[25rem] xl:w-[30rem]  border-[#010302] border-[1px] px-4 py-2" placeholder="Location")
                input(name="phone" class="w-full lg:w-[25rem] xl:w-[30rem] h-[3rem]  border-[#010302] border-[1px] px-4 py-2" placeholder="Phone number *optional")

                button(type="submit" class="bg-[#0093ff] text-white px-5 py-3  w-full h-auto text-xl font-medium  ") Add me to the Waiting List


    div(class="flex flex-col w-full mt-[2rem] lg:mt-[5rem]  items-center text-white  pt-[2rem] pb-[5rem] lg:pb-[12rem] bg-black")
            div(class="flex  lg:mt-12 w-auto relative flex-col text-3xl lg:text-[3rem] items-center  font-bold")
                //- nuxt-img(src="images/v2/star.png" width="60px" class=" absolute -top-[2rem] -right-[2rem]")
                //- nuxt-img(src="images/v2/star1.png" width="50px" class=" absolute top-[0rem] -right-[6rem]")
                //- nuxt-img(src="images/v2/star.png" width="40px" class=" absolute top-[2rem] -left-[3rem]")
                p(class="w-auto flex flex-wrap gap-x-3 items-center justify-center") Grow your 
                    span(class="text-[#7863FA]") Cleaning
                    span(class="hidden md:flex") Business
                    span(class="visible md:hidden") Business with Us

                p(class=" w-auto  hidden md:flex mt-6 ") With Us


            div(class="flex mt-16 lg:mt-28  w-full md:max-w-[86rem] flex-wrap justify-between items-start gap-y-12 lg:gap-y-[6rem] px-4 lg:px-0")
                div(class="flex flex-col items-center  w-[calc(50%-1rem)] lg:w-[calc(33%-2rem)] text-center ")
                    CustomerIcon(class="w-[4rem] h-[4rem] ")

                    p(class="text-lg lg:text-[2.0625rem] font-bold mt-[1.56rem] text-[#7863FA]") We'll bring you more customers
                    p(class="mt-[1rem] lg:px-[5.1rem]") The clients we bring are preciesly the ones you want. We'll help you grow your business and make more money.
                div(class="flex flex-col items-center w-[calc(50%-1rem)] lg:w-[calc(33%-2rem)] text-center ")
                    AccountingIcon(class="w-[4rem] h-[4rem] ")

                    p(class="text-lg lg:text-[2.0625rem] font-bold mt-[1.56rem] text-[#7863FA]") Automated Accounting
                    p(class="mt-[1rem] lg:px-[5.1rem]") Our platform provides easy-to-use tools for tracking earnings, expenses, invoices and automated auccounting, ensuring that you always have a clear picture of your financial health.
                div(class="flex flex-col items-center w-[calc(50%-1rem)] lg:w-[calc(33%-2rem)] text-center ")
                    <svg class="invert " width="65" height="64" viewBox="0 0 65 64" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M32.5002 46.6667C38.7135 46.6667 43.9668 42.7734 46.1268 37.3334H18.8735C21.0068 42.7734 26.2868 46.6667 32.5002 46.6667ZM23.1668 29.3334C24.2277 29.3334 25.2451 28.9119 25.9953 28.1618C26.7454 27.4117 27.1668 26.3942 27.1668 25.3334C27.1668 24.2725 26.7454 23.2551 25.9953 22.5049C25.2451 21.7548 24.2277 21.3334 23.1668 21.3334C22.106 21.3334 21.0885 21.7548 20.3384 22.5049C19.5883 23.2551 19.1668 24.2725 19.1668 25.3334C19.1668 26.3942 19.5883 27.4117 20.3384 28.1618C21.0885 28.9119 22.106 29.3334 23.1668 29.3334ZM41.8335 29.3334C42.8944 29.3334 43.9118 28.9119 44.6619 28.1618C45.4121 27.4117 45.8335 26.3942 45.8335 25.3334C45.8335 24.2725 45.4121 23.2551 44.6619 22.5049C43.9118 21.7548 42.8944 21.3334 41.8335 21.3334C40.7726 21.3334 39.7552 21.7548 39.0051 22.5049C38.2549 23.2551 37.8335 24.2725 37.8335 25.3334C37.8335 26.3942 38.2549 27.4117 39.0051 28.1618C39.7552 28.9119 40.7726 29.3334 41.8335 29.3334ZM32.5002 53.3334C26.8422 53.3334 21.416 51.0858 17.4152 47.085C13.4144 43.0842 11.1668 37.658 11.1668 32C11.1668 26.3421 13.4144 20.9159 17.4152 16.9151C21.416 12.9143 26.8422 10.6667 32.5002 10.6667C38.1581 10.6667 43.5843 12.9143 47.5851 16.9151C51.5859 20.9159 53.8335 26.3421 53.8335 32C53.8335 37.658 51.5859 43.0842 47.5851 47.085C43.5843 51.0858 38.1581 53.3334 32.5002 53.3334ZM32.5002 5.33337C17.7535 5.33337 5.8335 17.3334 5.8335 32C5.8335 39.0725 8.64301 45.8553 13.644 50.8562C16.1202 53.3325 19.0599 55.2967 22.2953 56.6368C25.5306 57.977 28.9982 58.6667 32.5002 58.6667C39.5726 58.6667 46.3554 55.8572 51.3563 50.8562C56.3573 45.8553 59.1668 39.0725 59.1668 32C59.1668 28.4981 58.4771 25.0305 57.137 21.7951C55.7968 18.5598 53.8326 15.6201 51.3563 13.1439C48.8801 10.6676 45.9404 8.70338 42.7051 7.36325C39.4697 6.02313 36.0021 5.33337 32.5002 5.33337Z" fill="#181526"/>
                    </svg>

                    p(class="text-lg lg:text-[2.0625rem] font-bold mt-[1.56rem] text-[#7863FA]") We care about your clients
                    p(class="mt-[1rem] text-base lg:px-[5.1rem] ") Your clients are the heart of your business, and at FreshAndTidy, they are our priority too. We provide you with tools and insights to understand your clients' needs better and tailor your services to meet their expectations. 

       
                div(class="flex flex-col items-center w-[calc(50%-1rem)] lg:w-[calc(33%-2rem)] text-center ")
                    StreamlineIcon(class="w-[4rem] h-[4rem] ")

                    p(class="text-lg lg:text-[2.0625rem] font-bold mt-[1.56rem] text-[#7863FA]") Streamline your business
                    p(class="mt-[1rem] lg:px-[5.1rem]") From scheduling and job management to client communication and feedback analysis, our platform is designed to optimize your workflow and save you valuable time. 
                div(class="flex flex-col items-center w-[calc(50%-1rem)] lg:w-[calc(33%-2rem)] text-center ")
                    MoneyIcon(class="w-[4rem] h-[4rem] ")

                    p(class="text-lg lg:text-[2.0625rem] font-bold mt-[1.56rem] text-[#7863FA]")  Competitive Compensation 
                    p(class="mt-[1rem] lg:px-[5.1rem]") Benefit from fair pricing models and timely payments for your services.
                div(class="flex flex-col items-center w-[calc(50%-1rem)] lg:w-[calc(33%-2rem)] text-center ") 
                    PlatformIcon(class="w-[4rem] h-[4rem] ")

                    p(class="text-lg lg:text-[2.0625rem] font-bold mt-[1.56rem] text-[#7863FA]") User-Friendly Platform
                    p(class="mt-[1rem] lg:px-[5.1rem]") Manage your bookings, payments, and client communications through our easy-to-use online portal.
    
           
            div(class="flex flex-col gap-y-[2rem] mt-[8rem] items-center")
                //- p(class="text-[1rem] font-bold tracking-wide") Find Your Local Maid Brigade
                button(@click="scrollToForm" class="w-[12rem]  gap-y-1 gap-x-2 bg-[#4E37E3] font-bold border-[1px] rounded-[0.4375rem] items-center text-white justify-center flex  px-[1.5rem] py-[1rem] text-center text-semibold ")
                    <svg width="19" height="18" viewBox="0 0 19 18" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M7.625 2.25C8.91793 2.25 10.1579 2.76361 11.0721 3.67785C11.9864 4.59209 12.5 5.83207 12.5 7.125C12.5 8.3325 12.0575 9.4425 11.33 10.2975L11.5325 10.5H12.125L15.875 14.25L14.75 15.375L11 11.625V11.0325L10.7975 10.83C9.9425 11.5575 8.8325 12 7.625 12C6.33207 12 5.09209 11.4864 4.17785 10.5721C3.26361 9.65791 2.75 8.41793 2.75 7.125C2.75 5.83207 3.26361 4.59209 4.17785 3.67785C5.09209 2.76361 6.33207 2.25 7.625 2.25ZM7.625 3.75C5.75 3.75 4.25 5.25 4.25 7.125C4.25 9 5.75 10.5 7.625 10.5C9.5 10.5 11 9 11 7.125C11 5.25 9.5 3.75 7.625 3.75Z" fill="#E8E8E9"/>
                    </svg>

                    p  Get started


    DemoClient
    

    
    
   
    div(class="flex flex-col w-full mt-[2rem] gap-y-12 lg:gap-y-24 items-center text-black  pt-[2rem] pb-[6rem] bg-white")
            div(class="flex  w-auto relative flex-col text-3xl lg:text-[3rem] items-center  font-bold")
                //- nuxt-img(src="images/v2/star.png" width="60px" class=" absolute -top-[2rem] -right-[2rem]")
                //- nuxt-img(src="images/v2/star1.png" width="50px" class=" absolute top-[0rem] -right-[6rem]")
                //- nuxt-img(src="images/v2/star.png" width="40px" class=" absolute top-[2rem] -left-[3rem]")
                p(class="w-auto flex flex-wrap gap-x-3 items-center justify-center text-3xl lg:text-[3rem] text-[#7863FA]") How
                    span(class="text-black")  does it Work?
                
            div(class="flex flex-col w-full lg:w-[50rem]  gap-y-6 items-start px-3 lg:px-0")
                    div(class="flex flex-col gap-y-1")
                        p(class="font-bold text-xl") Step 1: Sign Up and Get Onboard
                        div(class="flex flex-col items-start justify-start  ")
                            p(class="font-bold text-start")  Join and Showcase: 
                                span(class="font-normal") Quickly sign up and set up your profile to showcase your services, skills, and experience on partner's portal
                        
                    div(class="flex flex-col gap-y-1")        
                        p(class="font-bold text-xl") Step 2: We Match You with Ideal Clients
                        div(class="flex flex-col items-start justify-start ")
                            p(class="font-bold text-start") Client Matching: 
                                span(class="font-normal") Based on your service offerings and location, we connect you with clients who are looking for exactly what you provide using our Portal www.freshandtidy.nl

                    div(class="flex flex-col gap-y-1")
                        p(class="font-bold text-xl") Step 3: Simplified Financial Management
                        div(class="flex flex-col items-start justify-start ")
                            p(class="font-bold text-start") Track Finances: 
                                span(class="font-normal") Track and manage your finances effortlessly with automated tools for earnings, expenses, and invoicing.
                    div(class="flex flex-col gap-y-1")
                        p(class="font-bold text-xl") Step 4: Deliver Great Service and Build Relationships
                        div(class="flex flex-col items-start justify-start ")
                            p(class="font-bold text-start") Understand Client Needs:
                                span(class="font-normal") Understand and cater to client needs, ensuring top-notch service and building lasting relationships.
                       
                    div(class="flex flex-col gap-y-1")
                        p(class="font-bold text-xl") Step 5: Streamline Operations
                        div(class="flex flex-col items-start justify-start ")
                            p(class="font-bold text-start") Efficiency Tools: 
                                span(class="font-normal") Use our smart scheduling and job management features to optimize your time and resources.
                    div(class="flex flex-col gap-y-1")
                        p(class="font-bold text-xl") Step 6: Earn and Grow
                        div(class="flex flex-col items-start justify-start ")
                                p(class="font-bold text-start") Earnings and Expansion: 
                                    span(class="font-normal") Benefit from competitive earnings, prompt payments, and opportunities to expand your team and services.

                    div(class="flex flex-col gap-y-1")
                        p(class="font-bold text-xl") Step 7: Continuous Improvement and Support
                        div(class="flex flex-col items-start justify-start ") 
                                p(class="font-bold text-start") Feedback and Ratings: 
                                    span(class="font-normal") After each job, view client feedback to continuously improve your services.
                                p(class="font-bold text-start") Dedicated Support: 
                                    span(class="font-normal") Our support team is always here to assist you with any questions or guidance you need.
                    div(class="flex flex-col gap-y-1")
                        p(class="font-bold text-xl") Step 8: Expand Your Reach
                        div(class="flex flex-col items-start justify-start ")
                                p(class="font-bold text-start") Expand Your Reach: 
                                    span(class="font-normal") Grow your client base and scale your business, leveraging the insights and tools from FreshAndTidy.
    div(class="flex flex-col  items-center lg:-mt-8")
        //- p(class="text-[1rem] font-bold tracking-wide") Find Your Local Maid Brigade
        button(@click="scrollToForm" class="w-[12rem]  gap-y-1 gap-x-2 bg-[#4E37E3] font-bold border-[1px] rounded-[0.4375rem] items-center text-white justify-center flex  px-[1.5rem] py-[1rem] text-center text-semibold ")
            <svg width="19" height="18" viewBox="0 0 19 18" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M7.625 2.25C8.91793 2.25 10.1579 2.76361 11.0721 3.67785C11.9864 4.59209 12.5 5.83207 12.5 7.125C12.5 8.3325 12.0575 9.4425 11.33 10.2975L11.5325 10.5H12.125L15.875 14.25L14.75 15.375L11 11.625V11.0325L10.7975 10.83C9.9425 11.5575 8.8325 12 7.625 12C6.33207 12 5.09209 11.4864 4.17785 10.5721C3.26361 9.65791 2.75 8.41793 2.75 7.125C2.75 5.83207 3.26361 4.59209 4.17785 3.67785C5.09209 2.76361 6.33207 2.25 7.625 2.25ZM7.625 3.75C5.75 3.75 4.25 5.25 4.25 7.125C4.25 9 5.75 10.5 7.625 10.5C9.5 10.5 11 9 11 7.125C11 5.25 9.5 3.75 7.625 3.75Z" fill="#E8E8E9"/>
            </svg>

            p  Get started

    DemoPartners
    div(class="flex flex-col  items-center lg:-mt-8 pb-8")
        //- p(class="text-[1rem] font-bold tracking-wide") Find Your Local Maid Brigade
        button(@click="scrollToForm" class="w-[12rem]  gap-y-1 gap-x-2 bg-[#4E37E3] font-bold border-[1px] rounded-[0.4375rem] items-center text-white justify-center flex  px-[1.5rem] py-[1rem] text-center text-semibold ")
            <svg width="19" height="18" viewBox="0 0 19 18" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M7.625 2.25C8.91793 2.25 10.1579 2.76361 11.0721 3.67785C11.9864 4.59209 12.5 5.83207 12.5 7.125C12.5 8.3325 12.0575 9.4425 11.33 10.2975L11.5325 10.5H12.125L15.875 14.25L14.75 15.375L11 11.625V11.0325L10.7975 10.83C9.9425 11.5575 8.8325 12 7.625 12C6.33207 12 5.09209 11.4864 4.17785 10.5721C3.26361 9.65791 2.75 8.41793 2.75 7.125C2.75 5.83207 3.26361 4.59209 4.17785 3.67785C5.09209 2.76361 6.33207 2.25 7.625 2.25ZM7.625 3.75C5.75 3.75 4.25 5.25 4.25 7.125C4.25 9 5.75 10.5 7.625 10.5C9.5 10.5 11 9 11 7.125C11 5.25 9.5 3.75 7.625 3.75Z" fill="#E8E8E9"/>
            </svg>

            p  Get started
</template>

<script setup lang="ts">
import StreamlineIcon from "~icons/fluent/stream-24-regular";
import AccountingIcon from "~icons/mdi/account-cash-outline";
import CustomerIcon from "~icons/ic/outline-people";
import MoneyIcon from "~icons/healthicons/money-bag-outline";
import PlatformIcon from "~icons/gg/website";

const { setLocale } = useI18n();

const scrollToForm = () => {
  window.scrollTo({
    top: 0,
    behavior: "smooth",
  });
};

definePageMeta({
  // or middleware: 'auth'
  isSidebarOff: true,
});

// useSeoMeta({
//   title: "FreshAndTidy | Waiting List",
//   ogTitle: "FreshAndTidy | Waiting List",
//   description: "Waiting List",
//   ogDescription: "Waiting List",
//   ogImage: "",
//   twitterImage: "",
// });
</script>

<style lang="scss"></style>
